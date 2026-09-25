import { useEffect, useRef, useState } from 'react';
import {
  streamCopilotChat,
  type ChatMessage,
  type CopilotError,
  type CopilotSettings,
} from './api';
import { SessionCard, type SessionCardData } from './SessionCard';
import { useWorkspaceStore } from '../../stores';

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

export function CopilotPanel({ contextPath, settings, onOpenSettings }: CopilotPanelProps) {
  const { dispatchedTaskPrompt, clearDispatchedTask } = useWorkspaceStore();

  const [cards, setCards] = useState<SessionCardData[]>([]);
  const [input, setInput] = useState('');
  const [isStreaming, setIsStreaming] = useState(false);

  const abortRef = useRef<AbortController | null>(null);
  const transcriptRef = useRef<HTMLDivElement | null>(null);

  // Consume dispatched task prompt from Action Pill (R6)
  useEffect(() => {
    if (dispatchedTaskPrompt) {
      // eslint-disable-next-line react/set-state-in-effect
      setInput(dispatchedTaskPrompt);
      clearDispatchedTask();
    }
  }, [dispatchedTaskPrompt, clearDispatchedTask]);

  // Abort in-flight turn when the panel unmounts (R4)
  useEffect(() => () => abortRef.current?.abort(), []);

  useEffect(() => {
    const transcript = transcriptRef.current;
    if (transcript) {
      transcript.scrollTop = transcript.scrollHeight;
    }
  }, [cards]);

  const runTurn = async (promptText: string) => {
    const controller = new AbortController();
    abortRef.current = controller;

    const promptCardId = `prompt-${Date.now()}`;
    const answerCardId = `answer-${Date.now() + 1}`;

    const promptCard: SessionCardData = {
      id: promptCardId,
      kind: 'prompt',
      text: promptText,
      contextPath,
      timestamp: Date.now(),
    };

    const initialAnswerCard: SessionCardData = {
      id: answerCardId,
      kind: 'answer',
      text: '',
      isStreaming: true,
      timestamp: Date.now(),
    };

    setCards((prev) => [...prev, promptCard, initialAnswerCard]);
    setIsStreaming(true);

    // Build conversation history from completed cards
    const history: ChatMessage[] = [];
    for (const card of cards) {
      if (card.kind === 'prompt') {
        history.push({ role: 'user', text: card.text });
      } else if (card.kind === 'answer' && !card.isStreaming) {
        history.push({ role: 'model', text: card.text });
      }
    }
    history.push({ role: 'user', text: promptText });

    let currentAnswer = '';
    const failureHolder: { error: CopilotError | null } = { error: null };

    await streamCopilotChat(
      { messages: history, contextPath },
      (event) => {
        if (event.type === 'token') {
          currentAnswer += event.text;
          setCards((prev) =>
            prev.map((c) => (c.id === answerCardId ? { ...c, text: currentAnswer } : c))
          );
        } else if (event.type === 'error') {
          failureHolder.error = event.error;
        }
      },
      controller.signal
    );

    abortRef.current = null;
    setIsStreaming(false);

    if (controller.signal.aborted) {
      setCards((prev) =>
        prev.map((c) =>
          c.id === answerCardId
            ? { ...c, text: `${currentAnswer}\n\n_(stopped)_`, isStreaming: false }
            : c
        )
      );
      return;
    }

    if (failureHolder.error) {
      const failure = failureHolder.error;
      // Remove empty answer card and append failure card (R7)
      const failureCard: SessionCardData = {
        id: `failure-${Date.now()}`,
        kind: 'failure',
        code: failure.code,
        message: ERROR_HINTS[failure.code] ?? failure.message,
        canRetry: failure.code !== 'PROVIDER_NOT_CONFIGURED',
        timestamp: Date.now(),
      };

      setCards((prev) => [...prev.filter((c) => c.id !== answerCardId), failureCard]);
      setInput(promptText);
      return;
    }

    setCards((prev) => prev.map((c) => (c.id === answerCardId ? { ...c, isStreaming: false } : c)));
  };

  const submitComposer = () => {
    const prompt = input.trim();
    if (!prompt || isStreaming) {
      return;
    }
    setInput('');
    void runTurn(prompt);
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
    <aside className="astryx-sidebar-right astryx-copilot flex flex-col h-full">
      <div className="astryx-panel-header">
        <span>Conversational Co-Pilot</span>
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

      {/* Context header: document bound to turn (R1, S6) */}
      <div className="astryx-copilot-context">
        <span className="astryx-copilot-context-label">Context</span>
        {contextPath ? (
          <code className="astryx-copilot-context-path" title={contextPath}>
            {contextPath}
          </code>
        ) : (
          <span className="astryx-input-hint">No document bound — select an epic or file.</span>
        )}
      </div>

      <div
        className="astryx-copilot-transcript flex-1 overflow-y-auto p-4 flex flex-col gap-3"
        ref={transcriptRef}
      >
        {cards.length === 0 && !isStreaming && (
          <p className="astryx-input-hint text-xs text-[var(--text-muted)]">
            Ask the co-pilot to clarify, review or generate specifications for the active context.
          </p>
        )}

        {cards.map((card) => (
          <SessionCard key={card.id} card={card} onRetry={submitComposer} />
        ))}
      </div>

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
