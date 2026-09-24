# Plan: Workspace FS Scanner & File Read API

## Intent
Provide secure filesystem discovery and content reading endpoints with strict path traversal guards ensuring access is confined to the active workspace.

## Domain Model
- **Terms:** SSOT, Workspace Inspector
- **Entities:** WorkspaceTree, FileNode, PathGuard
- **Domain events:** WorkspaceScanned, FileReadRequested

## Business Rules
- **R1:** Path traversal attempts containing relative parents or resolving outside the canonical workspace root must be rejected immediately with HTTP 403 Forbidden.
- **R2:** Hidden directories (such as `.git`, `.cargo`, `target`) must be excluded from `/api/workspace/tree` responses unless explicitly queried.

## Examples
- **S1 — Workspace directory tree query:** Given a workspace initialized on disk, When a GET request is sent to `/api/workspace/tree`, Then the server responds with 200 containing a recursive JSON tree of directories and files.
- **S2 — Path traversal attack rejected:** Given the server is active, When a GET request is sent to `/api/workspace/file?path=../../etc/passwd`, Then the server responds with 403 Forbidden and error code `TRAVERSAL_DETECTED`.

## Slices
1. Filesystem scanner tree and safe file content endpoint with traversal guard — S1, S2, R1, R2

## Open Questions
- None. [resolved]
