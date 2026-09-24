import { useState } from 'react';
import './App.css';

interface FeatureEpic {
  id: string;
  title: string;
  state: 'Draft' | 'Ready' | 'WIP' | 'Done';
}

function App() {
  const [activeProject] = useState('liquid_ade');
  const [activeEpic] = useState<string>('epic_01_runtime_shell');

  const epics: FeatureEpic[] = [
    { id: 'epic_01_runtime_shell', title: 'Runtime Shell & Astryx Layout', state: 'WIP' },
    { id: 'epic_02_workspace_fs_scanner', title: 'Workspace FS Scanner', state: 'Draft' },
  ];

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
        </div>

        <div className="astryx-header-right">
          <span className="astryx-chip wip">WIP</span>
          <span className="astryx-kbd">⌘K</span>
        </div>
      </header>

      {/* 3-Column Layout Container */}
      <main className="astryx-main">
        {/* Column 1: Left Navigation Sidebar */}
        <aside className="astryx-sidebar-left">
          <div className="astryx-panel-header">
            <span>Workspace</span>
            <span className="astryx-kbd">⌥1</span>
          </div>

          <div className="astryx-nav-tree">
            <div className="astryx-nav-group-title">
              <span>01-workspace-inspector</span>
            </div>

            {epics.map((epic) => (
              <div
                key={epic.id}
                className={`astryx-nav-item ${epic.id === activeEpic ? 'active' : ''}`}
              >
                <span>{epic.title}</span>
                <span className={`astryx-chip ${epic.state.toLowerCase()}`}>{epic.state}</span>
              </div>
            ))}
          </div>
        </aside>

        {/* Column 2: Document Canvas */}
        <section className="astryx-canvas">
          <div className="astryx-doc-header">
            <div className="astryx-breadcrumbs">
              <span>features</span>
              <span>/</span>
              <span>01-workspace-inspector</span>
              <span>/</span>
              <span>epics</span>
              <span>/</span>
              <span>epic_01_runtime_shell</span>
            </div>

            <div className="astryx-doc-title-row">
              <h1 className="astryx-doc-title">Runtime Shell & Astryx Layout</h1>
              <span className="astryx-chip wip">WIP</span>
            </div>
          </div>

          <div className="astryx-card">
            <div className="astryx-card-title">Intent</div>
            <p className="astryx-prose">
              Provide an Axum web server embedding a React 19 static SPA that renders the
              foundational 3-column Astryx layout, initializes an embedded SQLite database for
              project registry and checkpoints, and exposes health check endpoints without external
              runtime dependencies.
            </p>
          </div>

          <div className="astryx-card">
            <div className="astryx-card-title">Implementation Tasks</div>
            <div className="astryx-task-list">
              <div className="astryx-task-item">
                <input type="checkbox" className="astryx-task-checkbox" checked readOnly />
                <span className="astryx-task-text">
                  TASK-01: Implement Axum server with rust-embed SPA fallback and /api/health
                </span>
                <span className="astryx-task-badge">S1, S2, R1, R2</span>
              </div>

              <div className="astryx-task-item">
                <input type="checkbox" className="astryx-task-checkbox" checked readOnly />
                <span className="astryx-task-text">
                  TASK-02: Implement React 19 Astryx shell layout component
                </span>
                <span className="astryx-task-badge">S2, R1</span>
              </div>

              <div className="astryx-task-item">
                <input type="checkbox" className="astryx-task-checkbox" checked readOnly />
                <span className="astryx-task-text">
                  TASK-03: Implement embedded SQLite database initialization and schema migration in
                  Rust
                </span>
                <span className="astryx-task-badge">S3, R3</span>
              </div>
            </div>
          </div>
        </section>

        {/* Column 3: Right Inspector Sidebar */}
        <aside className="astryx-sidebar-right">
          <div className="astryx-panel-header">
            <span>Inspector</span>
            <span className="astryx-kbd">⌥3</span>
          </div>

          <div className="astryx-inspector-section">
            <div className="astryx-card-title">Rules & Invariants</div>

            <div className="astryx-inspector-item">
              <div className="astryx-inspector-item-head">
                <span className="astryx-rule-tag">R1</span>
                <span className="astryx-chip ready">Enforced</span>
              </div>
              <p className="astryx-inspector-desc">
                Single binary with zero Node.js runtime dependency, serving embedded static assets
                via rust-embed.
              </p>
            </div>

            <div className="astryx-inspector-item">
              <div className="astryx-inspector-item-head">
                <span className="astryx-rule-tag">R2</span>
                <span className="astryx-chip ready">Enforced</span>
              </div>
              <p className="astryx-inspector-desc">
                All API routes must fail fast and return typed JSON responses with appropriate HTTP
                status codes.
              </p>
            </div>

            <div className="astryx-inspector-item">
              <div className="astryx-inspector-item-head">
                <span className="astryx-rule-tag">R3</span>
                <span className="astryx-chip ready">Enforced</span>
              </div>
              <p className="astryx-inspector-desc">
                Embedded SQLite database initializes at ~/.liquid/liquid.db with projects,
                checkpoints, and settings tables.
              </p>
            </div>
          </div>

          <div className="astryx-inspector-section">
            <div className="astryx-card-title">Acceptance Scenarios</div>

            <div className="astryx-inspector-item">
              <div className="astryx-inspector-item-head">
                <span className="astryx-scenario-tag">S1</span>
                <span className="astryx-chip ready">Passed</span>
              </div>
              <p className="astryx-inspector-desc">
                GET /api/health returns 200 {'{'}
                &quot;status&quot;:&quot;ok&quot;,&quot;app&quot;:&quot;liquid-ade&quot;{'}'}.
              </p>
            </div>

            <div className="astryx-inspector-item">
              <div className="astryx-inspector-item-head">
                <span className="astryx-scenario-tag">S2</span>
                <span className="astryx-chip ready">Passed</span>
              </div>
              <p className="astryx-inspector-desc">
                GET /unknown-path returns 200 serving index.html as SPA fallback.
              </p>
            </div>

            <div className="astryx-inspector-item">
              <div className="astryx-inspector-item-head">
                <span className="astryx-scenario-tag">S3</span>
                <span className="astryx-chip ready">Passed</span>
              </div>
              <p className="astryx-inspector-desc">
                Embedded SQLite database boots and initializes tables in ~/.liquid/liquid.db.
              </p>
            </div>
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
          <span>⌘K Command Palette</span>
          <span>•</span>
          <span>⌘Z Undo</span>
          <span>•</span>
          <span>/ Slash Menu</span>
        </div>
      </footer>
    </div>
  );
}

export default App;
