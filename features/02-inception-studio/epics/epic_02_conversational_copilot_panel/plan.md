# Plan: Conversational Co-Pilot Panel & Streaming Client

## Intent
Provide a dual-pane conversational co-pilot panel where the user interacts with an AI agent in a
chat pane while simultaneously viewing and focusing on the relevant specification document canvas.

The document canvas stops being a static mock and becomes a live projection of disk state: the
left navigation tree is fed by `/api/workspace/tree`, the canvas renders the markdown returned by
`/api/workspace/file`, and the co-pilot always knows which document the user is looking at. The
model is reached through the Multi-LLM Gateway, whose first concrete provider is the Google Gemini
REST API, proxied by the Rust binary over Server-Sent Events so that no credential ever reaches
the browser.

## Domain Model
- **Terms:** Conversational Co-Pilot, Document-as-UI, Multi-LLM Gateway, Workspace Inspector
- **Entities:** CopilotSession, ChatMessage, CanvasDocumentBinding, ProviderCredential
- **Domain events:** PromptSubmitted, TokenStreamReceived, DocumentContextBound, ProviderConfigured

## Business Rules
- **R1:** The co-pilot panel must maintain document context binding so every conversation turn
  knows the active file on the document canvas.
- **R2:** Responses from the model must stream incrementally without blocking the user interface
  main thread.
- **R3:** Provider credentials are server-side only. They are persisted in the local SQLite
  `settings` table and must never be returned to the client nor embedded in the SPA bundle; the
  settings read endpoint reports configuration state, never the secret.
- **R4:** The co-pilot endpoint fails fast with typed JSON errors: a missing credential is
  rejected before any outbound call, and an unreachable or failing provider is surfaced as a
  typed upstream error instead of a panic or an opaque 500.
- **R5:** Document context is resolved server-side through the existing workspace path guard, so a
  co-pilot turn can never read a file outside the active workspace root.

## Examples
- **S1 — Streaming co-pilot message:** Given a user opens the co-pilot panel with `product_vision.md` active on the canvas and a provider credential configured, When the user sends the prompt "Clarify our target audience", Then the server responds `200 text/event-stream` and the answer tokens arrive as incremental `token` events terminated by a `done` event.
- **S2 — Resilient error handling on model failure:** Given a configured provider whose inference backend is offline, When the user submits a message, Then the server responds `502` with error code `UPSTREAM_UNAVAILABLE` so the chat panel can show an inline error banner with a retry action without clearing the input.
- **S3 — Document context binding:** Given a workspace containing `product_vision.md`, When the user selects that file on the workspace tree and submits a co-pilot prompt, Then the canvas renders the file content and the prompt sent upstream carries the bound document path and its content as context.
- **S4 — Provider credential is never disclosed:** Given a credential saved through `POST /api/settings/copilot`, When a client requests `GET /api/settings/copilot`, Then the response reports `configured: true` with the selected model and contains no fragment of the stored key.
- **S5 — Missing credential rejected before any outbound call:** Given no provider credential configured, When the user submits a co-pilot prompt, Then the server responds `424` with error code `PROVIDER_NOT_CONFIGURED`.

## Slices
1. Live Document-as-UI canvas: workspace tree navigation, markdown rendering and canvas/co-pilot context binding — S3, R1
2. Server-side streaming gateway: SSE co-pilot endpoint proxying the Gemini REST API with typed failure modes — S1, S2, S5, R2, R4, R5
3. Provider credential settings: SQLite-backed, write-only credential persistence — S4, R3

Each slice is vertical: slice 1 spans disk read to rendered canvas, slice 2 spans HTTP request to
streamed token, slice 3 spans persistence to settings UI.

## Open Questions
- None. [resolved]
