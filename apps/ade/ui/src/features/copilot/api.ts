/**
 * Conversational Co-Pilot HTTP contracts.
 *
 * The browser never holds a provider credential (R3): it posts a conversation to the Rust
 * gateway and consumes the normalized Server-Sent Events stream it proxies back. Streaming is
 * read incrementally off a `ReadableStream`, so tokens render without blocking the main thread
 * (R2).
 */

export type ChatRole = 'user' | 'model';

export interface ChatMessage {
  role: ChatRole;
  text: string;
}

export interface CopilotSettings {
  provider: string;
  model: string;
  configured: boolean;
  credential_source: 'database' | 'environment' | 'none';
}

/** Typed failure surfaced to the panel, whether it came from HTTP or from an SSE `error` event. */
export interface CopilotError {
  code: string;
  message: string;
}

interface ApiError {
  error: string;
  message: string;
}

/** Normalized events emitted by `POST /api/copilot/chat`. */
export type CopilotStreamEvent =
  { type: 'token'; text: string } | { type: 'error'; error: CopilotError } | { type: 'done' };

async function readError(response: Response): Promise<CopilotError> {
  const detail = (await response.json().catch(() => null)) as ApiError | null;
  return {
    code: detail?.error ?? `HTTP_${response.status}`,
    message: detail?.message ?? `Request failed with status ${response.status}`,
  };
}

export async function fetchCopilotSettings(): Promise<CopilotSettings> {
  const response = await fetch('/api/settings/copilot');
  if (!response.ok) {
    throw new Error((await readError(response)).message);
  }
  return (await response.json()) as CopilotSettings;
}

/**
 * Persists the provider credential and model. `apiKey` is write-only: the response reports the
 * resulting configuration state and never echoes the secret back (R3, S4).
 */
export async function saveCopilotSettings(input: {
  apiKey?: string;
  model?: string;
}): Promise<CopilotSettings> {
  const response = await fetch('/api/settings/copilot', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ api_key: input.apiKey, model: input.model }),
  });
  if (!response.ok) {
    throw new Error((await readError(response)).message);
  }
  return (await response.json()) as CopilotSettings;
}

/** One `event:`/`data:` frame of an SSE stream. */
interface SseFrame {
  event: string;
  data: string;
}

/** Splits a raw SSE buffer on blank lines, returning complete frames and the pending remainder. */
function drainFrames(buffer: string): { frames: SseFrame[]; rest: string } {
  const normalized = buffer.replace(/\r\n/g, '\n');
  const blocks = normalized.split('\n\n');
  const rest = blocks.pop() ?? '';
  const frames: SseFrame[] = [];

  for (const block of blocks) {
    let event = 'message';
    const data: string[] = [];

    for (const line of block.split('\n')) {
      if (line.startsWith('event:')) {
        event = line.slice('event:'.length).trim();
      } else if (line.startsWith('data:')) {
        data.push(line.slice('data:'.length).trim());
      }
    }

    if (data.length > 0) {
      frames.push({ event, data: data.join('\n') });
    }
  }

  return { frames, rest };
}

function parseFrame(frame: SseFrame): CopilotStreamEvent | null {
  let payload: Record<string, unknown>;
  try {
    payload = JSON.parse(frame.data) as Record<string, unknown>;
  } catch {
    return {
      type: 'error',
      error: { code: 'MALFORMED_EVENT', message: 'The gateway sent an unreadable event' },
    };
  }

  switch (frame.event) {
    case 'token': {
      const text = typeof payload.text === 'string' ? payload.text : '';
      return text ? { type: 'token', text } : null;
    }
    case 'error':
      return {
        type: 'error',
        error: {
          code: typeof payload.error === 'string' ? payload.error : 'UPSTREAM_ERROR',
          message:
            typeof payload.message === 'string'
              ? payload.message
              : 'The co-pilot provider reported an error',
        },
      };
    case 'done':
      return { type: 'done' };
    default:
      return null;
  }
}

/**
 * Streams one co-pilot turn.
 *
 * `contextPath` is the workspace-relative path bound to the document canvas (R1); the gateway
 * resolves its content server-side through the workspace path guard (R5). A typed HTTP failure
 * (`PROVIDER_NOT_CONFIGURED`, `UPSTREAM_UNAVAILABLE`, `TRAVERSAL_DETECTED`, `EMPTY_PROMPT`) is
 * delivered as an `error` event so the caller has a single failure path to render.
 */
export async function streamCopilotChat(
  request: { messages: ChatMessage[]; contextPath: string | null },
  onEvent: (event: CopilotStreamEvent) => void,
  signal?: AbortSignal
): Promise<void> {
  let response: Response;
  try {
    response = await fetch('/api/copilot/chat', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        messages: request.messages,
        context_path: request.contextPath,
      }),
      signal,
    });
  } catch (cause) {
    if (signal?.aborted) {
      return;
    }
    onEvent({
      type: 'error',
      error: {
        code: 'GATEWAY_UNREACHABLE',
        message: cause instanceof Error ? cause.message : 'Could not reach the Liquid ADE gateway',
      },
    });
    return;
  }

  if (!response.ok) {
    onEvent({ type: 'error', error: await readError(response) });
    return;
  }

  if (!response.body) {
    onEvent({
      type: 'error',
      error: { code: 'STREAM_UNAVAILABLE', message: 'The gateway returned an empty stream' },
    });
    return;
  }

  const reader = response.body.getReader();
  const decoder = new TextDecoder();
  let buffer = '';

  try {
    for (;;) {
      const { done, value } = await reader.read();
      if (done) {
        break;
      }

      buffer += decoder.decode(value, { stream: true });
      const { frames, rest } = drainFrames(buffer);
      buffer = rest;

      for (const frame of frames) {
        const event = parseFrame(frame);
        if (event) {
          onEvent(event);
        }
      }
    }
  } catch (cause) {
    if (!signal?.aborted) {
      onEvent({
        type: 'error',
        error: {
          code: 'STREAM_INTERRUPTED',
          message: cause instanceof Error ? cause.message : 'The token stream was interrupted',
        },
      });
    }
  } finally {
    reader.releaseLock();
  }
}
