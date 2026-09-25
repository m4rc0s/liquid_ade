# Plan: Runtime Shell & Astryx Layout

## Intent
Provide an Axum web server embedding a React 19 static SPA that renders the foundational 3-column Astryx layout, initializes an embedded SQLite database for project registry and checkpoints, and exposes health check endpoints without external runtime dependencies.

## Domain Model
- **Terms:** Pure Static SPA, Workspace Inspector, Project Registry
- **Entities:** AppServer, AstryxShellLayout, SqliteStore
- **Domain events:** ShellBootstrapped, ServerHealthChecked, DatabaseMigrated

## Business Rules
- **R1:** The server must run as a single binary with zero Node.js runtime dependency, serving embedded static assets via rust-embed.
- **R2:** All API routes must fail fast and return typed JSON responses with appropriate HTTP status codes.
- **R3:** The embedded SQLite database must initialize automatically on startup under `~/.liquid/liquid.db` with schema migrations for projects, checkpoints, and settings.

## Examples
- **S1 — Health check returns status ok:** Given the Axum server is running on port 3000, When a GET request is sent to `/api/health`, Then the response status is 200 and the body is `{"status":"ok","app":"liquid-ade"}`.
- **S2 — Single page app fallback:** Given the Axum server is running, When a GET request is sent to `/unknown-path`, Then the server responds with 200 serving `index.html` as the SPA fallback.
- **S3 — Embedded SQLite engine initializes on boot:** Given the Axum server bootstraps, When the database engine initializes, Then the SQLite file at `~/.liquid/liquid.db` is opened and tables `projects`, `checkpoints`, and `settings` exist.

## Slices
1. Embedded Axum static SPA fallback and health check endpoint — S1, S2, R1, R2
2. Embedded SQLite engine initialization and migration slice — S3, R3

## Open Questions
- None. [resolved]
