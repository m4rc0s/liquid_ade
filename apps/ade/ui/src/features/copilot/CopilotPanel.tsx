import { useEffect, useRef, useState } from 'react';
import Markdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import {
  streamCopilotChat,
  type ChatMessage,
  type CopilotError,
  type CopilotSettings,
} from './api';

interface CopilotPanelProps {
  contextPath: string | null;
  settings: CopilotSettings | null;
  onOpenSettings: () => void;
}

/** Failure codes the panel resolves to an actionable hint instead of a raw provider message. */
const ERROR_HINTS: Record<string, string> = {
  PROVIDER_NOT_CONFIGURED: 'Configure a provider credential to start a conversation.',
  UPSTREAM_UNAVAILABLE: 'The provider is unreachable or rejected the request. Retry the turn.',
  TRAVERSAL_DETECTED: 'The bound document lies outside the workspace root and was refused.',
  EMPTY_PROMPT: 'Type a prompt before sending.',
};

/**
 * Right column: the conversational co-pilot pane of the dual-pane layout.
 *
 * Every turn carries the path bound to the document canvas (R1) and consumes the gateway's SSE
 * stream incrementally, so tokens paint as they arrive instead of blocking on a complete response
 * (R2). A failed turn is rolled back: the prompt returns to the composer untouched and an inline
 * banner offers a retry (S2).
 */
export function CopilotPanel({ contextPath, settings, onOpenSettings }: CopilotPanelProps) {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState('');
  const [streamingText, setStreamingText] = useState('');
  const [isStreaming, setIsStreaming] = useState(false);
  const [error, setError] = useState<CopilotError | null>(null);

  const abortRef = useRef<AbortController | null>(null);
  const transcriptRef = useRef<HTMLDivElement | null>(null);

  // Abort an in-flight turn when the panel unmounts so no stream outlives its consumer.
  useEffect(() => () => abortRef.current?.abort(), []);

  useEffect(() => {
    const transcript = transcriptRef.current;
    if (transcript) {
      transcript.scrollTop = transcript.scrollHeight;
    }
  }, [messages, streamingText]);

  const runTurn = async (conversation: ChatMessage[], prompt: string) => {
    const controller = new AbortController();
    abortRef.current = controller;

    setIsStreaming(true);
    setStreamingText('');
    setError(null);

    let answer = '';
    let failure: CopilotError | null = null;

    await streamCopilotChat(
      { messages: conversation, contextPath },
      (event) => {
        if (event.type === 'token') {
          answer += event.text;
          setStreamingText(answer);
        } else if (event.type === 'error') {
          failure = event.error;
        }
      },
      controller.signal
    );

    abortRef.current = null;
    setIsStreaming(false);
    setStreamingText('');

    if (controller.signal.aborted) {
      // A cancelled turn keeps whatever the model had already said, marked as partial.
      setMessages([...conversation, { role: 'model', text: `${answer}\n\n_(stopped)_` }]);
      return;
    }

    if (failure) {
      // Roll the turn back so the transcript only holds completed turns, and hand the prompt
      // back to the composer so the user never retypes it (S2).
      setMessages(conversation.slice(0, -1));
      setInput(prompt);
      setError(failure);
      return;
    }

    setMessages([...conversation, { role: 'model', text: answer }]);
  };

  /** Sends whatever the composer holds. Also the retry path, since a failed turn restores it. */
  const submitComposer = () => {
    const prompt = input.trim();
    if (!prompt || isStreaming) {
      return;
    }

    const conversation: ChatMessage[] = [...messages, { role: 'user', text: prompt }];
    setMessages(conversation);
    setInput('');
    void runTurn(conversation, prompt);
  };

  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault();
    submitComposer();
  };

  const handleKeyDown = (event: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      submitComposer();
    }
  };

  const isConfigured = settings?.configured ?? false;

  return (
    <aside className="astryx-sidebar-right astryx-copilot">
      <div className="astryx-panel-header">
        <span>Co-Pilot</span>
        <div className="astryx-panel-header-actions">
          <span className={`astryx-chip ${isConfigured ? 'ready' : 'draft'}`}>
            {settings ? (isConfigured ? settings.model : 'Not configured') : '…'}
          </span>
          <button
            type="button"
            className="astryx-icon-btn"
            onClick={onOpenSettings}
            title="Co-pilot provider settings"
          >
            ⚙
          </button>
        </div>
      </div>

      {/* Context header: the binding between the canvas document and every turn (R1). */}
      <div className="astryx-copilot-context">
        <span className="astryx-copilot-context-label">Context</span>
        {contextPath ? (
          <code className="astryx-copilot-context-path" title={contextPath}>
            {contextPath}
          </code>
        ) : (
          <span className="astryx-input-hint">
            No document bound — select a file on the workspace tree.
          </span>
        )}
      </div>

      <div className="astryx-copilot-transcript" ref={transcriptRef}>
        {messages.length === 0 && !isStreaming && (
          <p className="astryx-input-hint">
            Ask the co-pilot to clarify, review or rewrite the document on the canvas.
          </p>
        )}

        {messages.map((message, index) => (
          <div
            key={`${message.role}-${index}`}
            className={`astryx-chat-message ${message.role === 'user' ? 'user' : 'model'}`}
          >
            <span className="astryx-chat-role">{message.role === 'user' ? 'You' : 'Co-Pilot'}</span>
            {message.role === 'user' ? (
              <p className="astryx-chat-text">{message.text}</p>
            ) : (
              <div className="astryx-chat-text astryx-markdown compact">
                <Markdown remarkPlugins={[remarkGfm]}>{message.text}</Markdown>
              </div>
            )}
          </div>
        ))}

        {isStreaming && (
          <div className="astryx-chat-message model">
            <span className="astryx-chat-role">Co-Pilot</span>
            <div className="astryx-chat-text astryx-markdown compact">
              {streamingText ? (
                <Markdown remarkPlugins={[remarkGfm]}>{streamingText}</Markdown>
              ) : (
                <span className="astryx-input-hint">Streaming…</span>
              )}
              <span className="astryx-stream-caret" />
            </div>
          </div>
        )}
      </div>

      {error && (
        <div className="astryx-error-box astryx-error-box-actionable">
          <div className="astryx-error-detail">
            <span className="astryx-error-code">{error.code}</span>
            <span>{ERROR_HINTS[error.code] ?? error.message}</span>
          </div>
          {error.code === 'PROVIDER_NOT_CONFIGURED' ? (
            <button
              type="button"
              className="astryx-btn astryx-btn-secondary"
              onClick={onOpenSettings}
            >
              Configure
            </button>
          ) : (
            <button
              type="button"
              className="astryx-btn astryx-btn-secondary"
              onClick={submitComposer}
              disabled={input.trim() === ''}
            >
              Retry
            </button>
          )}
        </div>
      )}

      <form className="astryx-copilot-composer" onSubmit={handleSubmit}>
        <textarea
          className="astryx-input astryx-copilot-input"
          rows={3}
          placeholder="Ask the co-pilot…"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          disabled={isStreaming}
        />
        <div className="astryx-copilot-composer-actions">
          <span className="astryx-input-hint">⏎ send · ⇧⏎ newline</span>
          {isStreaming ? (
            <button
              type="button"
              className="astryx-btn astryx-btn-secondary"
              onClick={() => abortRef.current?.abort()}
            >
              Stop
            </button>
          ) : (
            <button
              type="submit"
              className="astryx-btn astryx-btn-primary"
              disabled={input.trim() === ''}
            >
              Send
            </button>
          )}
        </div>
      </form>
    </aside>
  );
}
