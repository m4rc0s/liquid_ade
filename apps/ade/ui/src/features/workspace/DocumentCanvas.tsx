import Markdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import type { FileContent } from './api';

interface DocumentCanvasProps {
  activeDocument: FileContent | null;
  isLoading: boolean;
  error: string | null;
  onRetry: () => void;
}

/** Extensions the canvas renders as rich Markdown; everything else falls back to plain source. */
const MARKDOWN_EXTENSIONS = /\.(md|markdown)$/i;

function isMarkdown(path: string): boolean {
  return MARKDOWN_EXTENSIONS.test(path);
}

/**
 * Splits the document into the title shown in the canvas header and the body rendered below it.
 *
 * A leading `#` heading is promoted to the header chrome and removed from the body, so the title
 * is never printed twice; any other document falls back to its file name.
 */
function splitTitle(doc: FileContent): { title: string; body: string } {
  const heading = doc.content.match(/^\s*#\s+(.+?)\s*$/m);

  if (heading && heading.index !== undefined && doc.content.slice(0, heading.index).trim() === '') {
    return {
      title: heading[1].trim(),
      body: doc.content.slice(heading.index + heading[0].length).replace(/^\n+/, ''),
    };
  }

  return { title: doc.path.split('/').pop() ?? doc.path, body: doc.content };
}

/**
 * Center column: the Document-as-UI canvas.
 *
 * The canvas is a projection of disk state — its content always comes from `/api/workspace/file`,
 * never from a local mock — and the path it renders is the document bound to the co-pilot turn
 * (R1).
 */
export function DocumentCanvas({ activeDocument, isLoading, error, onRetry }: DocumentCanvasProps) {
  if (isLoading) {
    return (
      <section className="astryx-canvas">
        <div className="astryx-canvas-placeholder">Reading document…</div>
      </section>
    );
  }

  if (error) {
    return (
      <section className="astryx-canvas">
        <div className="astryx-error-box astryx-error-box-actionable">
          <span>{error}</span>
          <button type="button" className="astryx-btn astryx-btn-secondary" onClick={onRetry}>
            Retry
          </button>
        </div>
      </section>
    );
  }

  if (!activeDocument) {
    return (
      <section className="astryx-canvas">
        <div className="astryx-canvas-empty">
          <h2 className="astryx-canvas-empty-title">Document-as-UI</h2>
          <p className="astryx-prose">
            Select a specification on the workspace tree to project it onto the canvas. The co-pilot
            always answers with the document you are looking at bound as context.
          </p>
        </div>
      </section>
    );
  }

  const segments = activeDocument.path.split('/').filter(Boolean);
  const { title, body } = splitTitle(activeDocument);

  return (
    <section className="astryx-canvas">
      <div className="astryx-doc-header">
        <div className="astryx-breadcrumbs">
          {segments.map((segment, index) => (
            <span key={`${segment}-${index}`} className="astryx-breadcrumb-segment">
              {index > 0 && <span className="astryx-breadcrumb-sep">/</span>}
              <span>{segment}</span>
            </span>
          ))}
        </div>

        <div className="astryx-doc-title-row">
          <h1 className="astryx-doc-title">{title}</h1>
          <span className="astryx-doc-meta">{activeDocument.content.length} chars</span>
        </div>
      </div>

      {isMarkdown(activeDocument.path) ? (
        <article className="astryx-markdown">
          <Markdown remarkPlugins={[remarkGfm]}>{body}</Markdown>
        </article>
      ) : (
        <pre className="astryx-source-view">
          <code>{activeDocument.content}</code>
        </pre>
      )}
    </section>
  );
}
