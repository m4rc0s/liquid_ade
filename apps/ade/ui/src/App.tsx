import { useCallback, useEffect, useState } from 'react';
import './App.css';
import {
  fetchWorkspaceFile,
  fetchWorkspaceTree,
  isRenderableDocument,
  type FileContent,
  type FileNode,
} from './features/workspace/api';
import { WorkspaceTree } from './features/workspace/WorkspaceTree';
import { DocumentCanvas } from './features/workspace/DocumentCanvas';
import { CopilotPanel } from './features/copilot/CopilotPanel';
import { CopilotSettingsDialog } from './features/copilot/CopilotSettingsDialog';
import { fetchCopilotSettings, type CopilotSettings } from './features/copilot/api';

/** Document opened on the canvas when the workspace tree first loads, when present. */
const PREFERRED_ENTRY_DOCUMENT = 'product_vision.md';

function errorMessage(cause: unknown, fallback: string): string {
  return cause instanceof Error ? cause.message : fallback;
}

/** Depth-first search for the entry document, so a fresh session never opens an empty canvas. */
function findNode(node: FileNode, predicate: (candidate: FileNode) => boolean): FileNode | null {
  if (predicate(node)) {
    return node;
  }
  for (const child of node.children ?? []) {
    const found = findNode(child, predicate);
    if (found) {
      return found;
    }
  }
  return null;
}

function App() {
  const [tree, setTree] = useState<FileNode | null>(null);
  const [isTreeLoading, setIsTreeLoading] = useState(true);
  const [treeError, setTreeError] = useState<string | null>(null);

  const [activePath, setActivePath] = useState<string | null>(null);
  const [activeDocument, setActiveDocument] = useState<FileContent | null>(null);
  const [isDocumentLoading, setIsDocumentLoading] = useState(false);
  const [documentError, setDocumentError] = useState<string | null>(null);

  const [settings, setSettings] = useState<CopilotSettings | null>(null);
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);

  const [isProjectModalOpen, setIsProjectModalOpen] = useState(false);
  const [activeProject, setActiveProject] = useState('liquid_ade');
  const [newProjectName, setNewProjectName] = useState('');
  const [newProjectPath, setNewProjectPath] = useState('');
  const [modalError, setModalError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const openDocument = useCallback(async (path: string) => {
    setActivePath(path);
    setIsDocumentLoading(true);
    setDocumentError(null);

    try {
      setActiveDocument(await fetchWorkspaceFile(path));
    } catch (cause) {
      setActiveDocument(null);
      setDocumentError(errorMessage(cause, `Could not read ${path}`));
    } finally {
      setIsDocumentLoading(false);
    }
  }, []);

  const loadTree = useCallback(async () => {
    setIsTreeLoading(true);
    setTreeError(null);

    try {
      const root = await fetchWorkspaceTree();
      setTree(root);
      setActiveProject(root.name || 'workspace');

      const entry = findNode(root, (node) => node.name === PREFERRED_ENTRY_DOCUMENT);
      if (entry) {
        void openDocument(entry.path);
      }
    } catch (cause) {
      setTree(null);
      setTreeError(errorMessage(cause, 'Could not scan the workspace'));
    } finally {
      setIsTreeLoading(false);
    }
  }, [openDocument]);

  const loadSettings = useCallback(async () => {
    try {
      setSettings(await fetchCopilotSettings());
    } catch {
      // A settings read failure must not block the shell; the panel renders as unconfigured and
      // the co-pilot endpoint still reports PROVIDER_NOT_CONFIGURED on the first turn.
      setSettings(null);
    }
  }, []);

  // Mount-time synchronization with the two external systems the shell projects: the workspace
  // on disk and the gateway's provider configuration. Both loaders flip their own loading flag
  // before awaiting, which is the documented exception to `react/set-state-in-effect`.
  useEffect(() => {
    // eslint-disable-next-line react/set-state-in-effect
    void loadTree();
    void loadSettings();
  }, [loadTree, loadSettings]);

  const handleSelect = (node: FileNode) => {
    if (!isRenderableDocument(node)) {
      setActivePath(node.path);
      setActiveDocument(null);
      setDocumentError(`${node.name} is not a document the canvas can render.`);
      return;
    }
    void openDocument(node.path);
  };

  const isSlugValid = (s: string) => /^[a-z0-9][a-z0-9_-]*$/.test(s);

  const handleCreateProject = async (e: React.FormEvent) => {
    e.preventDefault();
    setModalError(null);

    const name = newProjectName.trim();
    const path = newProjectPath.trim();

    if (!name) {
      setModalError('Project slug is required');
      return;
    }

    if (!isSlugValid(name)) {
      setModalError(
        'Project slug must match ^[a-z0-9][a-z0-9_-]*$ (lowercase, digits, underscores, hyphens)'
      );
      return;
    }

    if (!path) {
      setModalError('Project directory path is required');
      return;
    }

    setIsSubmitting(true);
    try {
      const res = await fetch('/api/workspace/new', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, path }),
      });

      const data = await res.json();
      if (!res.ok) {
        setModalError(data.message || 'Failed to create workspace');
      } else {
        setActiveProject(data.name || name);
        setIsProjectModalOpen(false);
        setNewProjectName('');
        setNewProjectPath('');
      }
    } catch (err: unknown) {
      setModalError(errorMessage(err, 'Network error'));
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="astryx-shell">
      {/* Top Brand Header */}
      <header className="astryx-header">
        <div className="astryx-brand">
          <div className="astryx-logo-glyph">L</div>
          <span className="astryx-brand-title">Liquid ADE</span>
        </div>

        <div className="astryx-header-center">
          <div className="astryx-project-badge">
            <span className="astryx-status-dot"></span>
            <span>Project: {activeProject}</span>
          </div>
          <button
            type="button"
            className="astryx-btn astryx-btn-secondary"
            onClick={() => {
              setIsProjectModalOpen(true);
              setModalError(null);
            }}
          >
            + New Project
          </button>
        </div>

        <div className="astryx-header-right">
          <span className={`astryx-chip ${settings?.configured ? 'ready' : 'draft'}`}>
            {settings?.configured ? 'Co-Pilot Ready' : 'Co-Pilot Off'}
          </span>
          <span className="astryx-kbd">⌘K</span>
        </div>
      </header>

      {/* 3-Column Layout: workspace tree · document canvas · co-pilot */}
      <main className="astryx-main">
        <aside className="astryx-sidebar-left">
          <div className="astryx-panel-header">
            <span>Workspace</span>
            <span className="astryx-kbd">⌥1</span>
          </div>

          <WorkspaceTree
            root={tree}
            activePath={activePath}
            isLoading={isTreeLoading}
            error={treeError}
            onSelect={handleSelect}
            onRetry={() => void loadTree()}
          />
        </aside>

        <DocumentCanvas
          activeDocument={activeDocument}
          isLoading={isDocumentLoading}
          error={documentError}
          onRetry={() => activePath && void openDocument(activePath)}
        />

        <CopilotPanel
          contextPath={activeDocument?.path ?? null}
          settings={settings}
          onOpenSettings={() => setIsSettingsOpen(true)}
        />
      </main>

      {/* Bottom Status Bar */}
      <footer className="astryx-footer">
        <div className="astryx-footer-left">
          <div className="astryx-footer-indicator">
            <span className="astryx-status-dot"></span>
            <span>ACP Engine: Online (127.0.0.1:3000)</span>
          </div>
          <span>Storage: ~/.liquid/liquid.db</span>
        </div>

        <div className="astryx-footer-right">
          <span>{activePath ?? 'no document bound'}</span>
          <span>•</span>
          <span>⌘K Command Palette</span>
          <span>•</span>
          <span>/ Slash Menu</span>
        </div>
      </footer>

      {isSettingsOpen && (
        <CopilotSettingsDialog
          settings={settings}
          onClose={() => setIsSettingsOpen(false)}
          onSaved={(saved) => {
            setSettings(saved);
            setIsSettingsOpen(false);
          }}
        />
      )}

      {/* New Project Dialog Modal */}
      {isProjectModalOpen && (
        <div className="astryx-modal-overlay">
          <div className="astryx-modal" role="dialog" aria-modal="true">
            <div className="astryx-modal-header">
              <span className="astryx-modal-title">New SCPE Product Workspace</span>
              <button
                type="button"
                className="astryx-modal-close"
                onClick={() => setIsProjectModalOpen(false)}
              >
                ✕
              </button>
            </div>

            <form onSubmit={handleCreateProject}>
              <div className="astryx-modal-body">
                {modalError && <div className="astryx-error-box">{modalError}</div>}

                <div className="astryx-form-group">
                  <label htmlFor="project-slug-input" className="astryx-label">
                    Project Slug
                  </label>
                  <input
                    id="project-slug-input"
                    type="text"
                    className="astryx-input"
                    placeholder="e.g. mobile-banking"
                    value={newProjectName}
                    onChange={(e) => setNewProjectName(e.target.value)}
                    required
                  />
                  <span className="astryx-input-hint">
                    Canonical slug format: lowercase letters, numbers, hyphens, underscores.
                  </span>
                </div>

                <div className="astryx-form-group">
                  <label htmlFor="project-path-input" className="astryx-label">
                    Target Directory Path
                  </label>
                  <input
                    id="project-path-input"
                    type="text"
                    className="astryx-input"
                    placeholder="e.g. /home/user/projects/mobile-banking"
                    value={newProjectPath}
                    onChange={(e) => setNewProjectPath(e.target.value)}
                    required
                  />
                  <span className="astryx-input-hint">
                    Destination path on local filesystem. Must be an empty or new folder.
                  </span>
                </div>
              </div>

              <div className="astryx-modal-footer">
                <button
                  type="button"
                  className="astryx-btn astryx-btn-secondary"
                  onClick={() => setIsProjectModalOpen(false)}
                  disabled={isSubmitting}
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  className="astryx-btn astryx-btn-primary"
                  disabled={isSubmitting}
                >
                  {isSubmitting ? 'Creating...' : 'Create Workspace'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
