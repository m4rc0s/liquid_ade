# Plan: Project Scaffold Generator & New Workspace Flow

## Intent
Provide an interactive UI dialog and backend API endpoint to scaffold a new product project on disk initialized with canonical SCPE structure.

## Domain Model
- **Terms:** SCPE, Inception Studio
- **Entities:** ProjectScaffolder, WorkspaceManifest
- **Domain events:** ProjectCreated, ScaffoldGenerated

## Business Rules
- **R1:** Project names must match regex `^[a-z0-9][a-z0-9_-]*$` and cannot overwrite existing non-empty target paths without explicit confirmation.
- **R2:** Every scaffolded project must contain 100% of the canonical root files and directories required by SCPE v0.3.0.

## Examples
- **S1 — Scaffold new canonical SCPE project:** Given an empty target directory on disk, When a POST request is sent to `/api/workspace/new` with `{"name":"new-product","path":"/tmp/new-product"}`, Then the response status is 201 Created and all canonical SCPE files exist in the destination.
- **S2 — Reject invalid project slug:** Given an active server, When a POST request is sent to `/api/workspace/new` with `{"name":"Invalid Name!","path":"/tmp/invalid"}`, Then the response status is 400 Bad Request with validation error message.

## Slices
1. Scaffolding endpoint and UI modal for new project creation — S1, S2, R1, R2

## Open Questions
- None. [resolved]
