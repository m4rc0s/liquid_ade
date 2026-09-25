import { useCallback, useEffect, useState } from 'react';
import './App.css';
import { WorkspaceTree } from './features/workspace/WorkspaceTree';
import { DocumentCanvas } from './features/workspace/DocumentCanvas';
import { CopilotPanel } from './features/copilot/CopilotPanel';
import { CopilotSettingsDialog } from './features/copilot/CopilotSettingsDialog';
import { fetchCopilotSettings, type CopilotSettings } from './features/copilot/api';
import { ModeRail } from './features/scpe/ModeRail';
import { SpecNavigator } from './features/scpe/SpecNavigator';
import { SpecCanvas } from './features/scpe/SpecCanvas';
import { SpecInspector } from './features/scpe/SpecInspector';
import { useWorkspaceStore } from './stores';
import { Chip } from './components';

function errorMessage(cause: unknown, fallback: string): string {
  return cause instanceof Error ? cause.message : fallback;
}

function App() {
  const {
    activeMode,
    tree,
    isTreeLoading,
    treeError,
    activePath,
    activeDocument,
    isDocumentLoading,
    documentError,
    activeEpicDetail,
    activeRightTab,
    setActiveRightTab,
    activeProject,
    loadTree,
    loadOutline,
    openDocument,
    selectNode,
    setActiveProject,
  } = useWorkspaceStore();

  const [settings, setSettings] = useState<CopilotSettings | null>(null);
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);

  const [isProjectModalOpen, setIsProjectModalOpen] = useState(false);
  const [newProjectName, setNewProjectName] = useState('');
  const [newProjectPath, setNewProjectPath] = useState('');
  const [modalError, setModalError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const loadSettings = useCallback(async () => {
    try {
      setSettings(await fetchCopilotSettings());
    } catch {
      setSettings(null);
    }
  }, []);

  // Mount-time synchronization with workspace on disk, SCPE outline, and copilot settings
  useEffect(() => {
    void loadTree();
    void loadOutline();
    // eslint-disable-next-line react/set-state-in-effect
    void loadSettings();
  }, [loadTree, loadOutline, loadSettings]);

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
        void loadTree();
        void loadOutline();
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
          <Chip variant={settings?.configured ? 'ready' : 'draft'}>
            {settings?.configured ? 'Co-Pilot Ready' : 'Co-Pilot Off'}
          </Chip>
          <span className="astryx-kbd">⌘K</span>
        </div>
      </header>

      {/* Main Body: Mode Rail + Navigator + Canvas + Context Panel */}
      <main className="astryx-main">
        {/* 48px Mode Rail (R1) */}
        <ModeRail />

        {/* Left Column: Spec Navigator (default) or Filesystem Tree (R1) */}
        <aside className="astryx-sidebar-left">
          <div className="astryx-panel-header">
            <span>{activeMode === 'specs' ? 'Living Specs' : 'Filesystem'}</span>
            <span className="astryx-kbd">{activeMode === 'specs' ? '⌥S' : '⌥F'}</span>
          </div>

          {activeMode === 'specs' ? (
            <SpecNavigator />
          ) : (
            <WorkspaceTree
              root={tree}
              activePath={activePath}
              isLoading={isTreeLoading}
              error={treeError}
              onSelect={selectNode}
              onRetry={() => void loadTree()}
            />
          )}
        </aside>

        {/* Center Column: Living Spec Canvas (R3) or Document Canvas */}
        {activeEpicDetail ? (
          <SpecCanvas />
        ) : (
          <DocumentCanvas
            activeDocument={activeDocument}
            isLoading={isDocumentLoading}
            error={documentError}
            onRetry={() => activePath && void openDocument(activePath)}
          />
        )}

        {/* Right Column: Tabbed Inspector / Conversational Co-Pilot (R4) */}
        <aside className="astryx-sidebar-right w-[380px] min-w-[320px] p-0 flex flex-col h-full bg-[var(--bg-surface)] border-l border-[var(--border-subtle)]">
          <div className="astryx-tabs-header">
            <button
              type="button"
              onClick={() => setActiveRightTab('copilot')}
              className={`astryx-tab-btn ${activeRightTab === 'copilot' ? 'active' : ''}`}
            >
              Co-Pilot
            </button>
            <button
              type="button"
              onClick={() => setActiveRightTab('inspector')}
              className={`astryx-tab-btn ${activeRightTab === 'inspector' ? 'active' : ''}`}
            >
              Spec Inspector
            </button>
          </div>

          <div className="flex-1 overflow-hidden relative">
            {/* CopilotPanel stays mounted so in-flight SSE streams are preserved on tab toggle (R4) */}
            <div className={activeRightTab === 'copilot' ? 'h-full flex flex-col' : 'hidden'}>
              <CopilotPanel
                contextPath={activePath}
                settings={settings}
                onOpenSettings={() => setIsSettingsOpen(true)}
              />
            </div>

            {activeRightTab === 'inspector' && (
              <div className="h-full overflow-y-auto">
                <SpecInspector />
              </div>
            )}
          </div>
        </aside>
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
