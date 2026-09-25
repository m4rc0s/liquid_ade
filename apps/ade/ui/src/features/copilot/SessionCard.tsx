import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { AlertCircle, RotateCcw, User, Bot, FileCode } from 'lucide-react';

export type SessionCardData =
  | {
      id: string;
      kind: 'prompt';
      text: string;
      contextPath?: string | null;
      timestamp: number;
    }
  | {
      id: string;
      kind: 'answer';
      text: string;
      isStreaming?: boolean;
      timestamp: number;
    }
  | {
      id: string;
      kind: 'failure';
      code: string;
      message: string;
      canRetry?: boolean;
      timestamp: number;
    };

export interface SessionCardProps {
  card: SessionCardData;
  onRetry?: () => void;
}

export function SessionCard({ card, onRetry }: SessionCardProps) {
  switch (card.kind) {
    case 'prompt':
      return (
        <div className="astryx-chat-message user flex flex-col gap-1.5 p-3 rounded-lg bg-indigo-950/20 border border-indigo-500/20">
          <div className="flex items-center justify-between text-[10px] font-bold uppercase tracking-wider text-[var(--text-muted)]">
            <div className="flex items-center gap-1.5 text-indigo-400">
              <User size={12} strokeWidth={2} />
              <span>You</span>
            </div>
            {card.contextPath && (
              <span className="flex items-center gap-1 font-mono text-[10px] text-zinc-400 normal-case bg-black/30 px-1.5 py-0.5 rounded">
                <FileCode size={10} />
                <span className="truncate max-w-[160px]">{card.contextPath}</span>
              </span>
            )}
          </div>
          <div className="text-xs leading-relaxed text-[var(--text-primary)] whitespace-pre-wrap">
            {card.text}
          </div>
        </div>
      );

    case 'answer':
      return (
        <div className="astryx-chat-message model flex flex-col gap-1.5 p-3 rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)]">
          <div className="flex items-center justify-between text-[10px] font-bold uppercase tracking-wider text-[var(--text-muted)]">
            <div className="flex items-center gap-1.5 text-emerald-400">
              <Bot size={12} strokeWidth={2} />
              <span>Co-Pilot</span>
            </div>
          </div>
          <div className="astryx-markdown compact text-xs leading-relaxed text-[var(--text-secondary)]">
            <ReactMarkdown remarkPlugins={[remarkGfm]}>{card.text}</ReactMarkdown>
            {card.isStreaming && <span className="astryx-stream-caret" aria-hidden="true" />}
          </div>
        </div>
      );

    case 'failure':
      return (
        <div className="astryx-error-box flex flex-col gap-2 p-3 rounded-lg bg-rose-950/20 border border-rose-500/30 text-rose-300">
          <div className="flex items-center justify-between gap-2">
            <div className="flex items-center gap-1.5 text-xs font-semibold">
              <AlertCircle size={14} className="text-rose-400" />
              <span className="font-mono text-[10px] tracking-wide uppercase px-1.5 py-0.5 rounded bg-rose-950/50 border border-rose-800/40">
                {card.code}
              </span>
            </div>
            {card.canRetry && onRetry && (
              <button
                type="button"
                onClick={onRetry}
                className="flex items-center gap-1 px-2 py-0.5 rounded text-xs bg-rose-900/40 hover:bg-rose-900/60 border border-rose-700/50 text-rose-200 transition-colors"
              >
                <RotateCcw size={11} />
                <span>Retry</span>
              </button>
            )}
          </div>
          <div className="text-xs text-rose-200 leading-snug">{card.message}</div>
        </div>
      );
  }
}
