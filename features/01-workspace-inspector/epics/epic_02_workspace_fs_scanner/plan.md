# Plan: Workspace FS Scanner & File Read API

## Intent
Provide secure filesystem discovery, content reading/writing endpoints, and a project registry API backed by SQLite allowing users to create new SCPE projects, open existing projects, and switch workspaces with strict traversal guards.

## Domain Model
- **Terms:** SSOT, Workspace Inspector, Project Registry
- **Entities:** WorkspaceTree, FileNode, PathGuard, ProjectRecord
- **Domain events:** WorkspaceScanned, FileReadRequested, ProjectCreated, ProjectOpened

## Business Rules
- **R1:** Path traversal attempts containing relative parents or resolving outside the canonical workspace root must be rejected immediately with HTTP 403 Forbidden.
- **R2:** Hidden directories (such as `.git`, `.cargo`, `target`) must be excluded from `/api/workspace/tree` responses unless explicitly queried.
- **R3:** Creating or opening a project must persist the absolute path and metadata in SQLite and verify directory accessibility.

## Examples
- **S1 — Workspace directory tree query:** Given a workspace initialized on disk, When a GET request is sent to `/api/workspace/tree`, Then the server responds with 200 containing a recursive JSON tree of directories and files.
- **S2 — Path traversal attack rejected:** Given the server is active, When a GET request is sent to `/api/workspace/file?path=../../etc/passwd`, Then the server responds with 403 Forbidden and error code `TRAVERSAL_DETECTED`.
- **S3 — Register and switch active project:** Given a valid project path on disk, When a POST request is sent to `/api/projects/open` with `{"path":"/path/to/project"}`, Then the project is registered in SQLite with updated `last_opened_at` and the active workspace is set.

## Slices
1. Filesystem scanner tree and safe file content endpoint with traversal guard — S1, S2, R1, R2
2. Project registry endpoints for opening, creating, and switching active workspaces — S3, R3

## Open Questions
- None. [resolved]
