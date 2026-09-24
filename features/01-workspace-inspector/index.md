# Feature: Workspace Inspector & Runtime Shell

- **goal:** Provide a high-performance single-binary runtime shell and secure filesystem reader for inspecting SCPE workspaces
- **users:** Senior Architects, Product Engineers, Tech Leads, and Autonomous Agents
- **bounded_context:** Workspace Navigation & Inspection
- **business_value:** Delivers the foundational Day-1 ADE runtime that boots locally, embeds the static React 19 SPA, and renders workspace specifications directly from disk

## Scope
- Embedding the React 19 static SPA into the Axum binary via `rust-embed`.
- Serving static assets with client-side SPA routing fallback and health diagnostics (`/api/health`).
- Implementing the 3-column Astryx layout (Left: Navigation & File Tree, Center: Document Canvas, Right: Inspector Panel).
- Path traversal guard confining filesystem access strictly to the canonical workspace root.
- Scanning workspace directory trees (`/api/workspace/tree`) and reading file contents (`/api/workspace/file`).

## Out of Scope
- Conversational AI streaming and co-pilot panels (deferred to Phase 2: Inception Studio).
- Live WebSocket file watching with echo suppression (deferred to Phase 3: Live File Sync).
- Sandboxed code execution in Podman (deferred to Phase 7: Podman Sandbox Runner).

## Epics

- [epic_01_runtime_shell](epics/epic_01_runtime_shell/index.md) — Draft
- [epic_02_workspace_fs_scanner](epics/epic_02_workspace_fs_scanner/index.md) — Draft
