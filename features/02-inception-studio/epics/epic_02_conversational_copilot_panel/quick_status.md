# Status: Conversational Co-Pilot Panel & Streaming Client

- **state:** Done
- **confidence:** high
- **updated:** 2026-09-25

## Notes
Readiness Gate passed with 0 blocking issues and was approved by the Tech Lead. All 3 vertical
slices are delivered and verified (4/4 tasks).

- Slice 1 — Live Document-as-UI canvas: `WorkspaceTree` projects `/api/workspace/tree`,
  `DocumentCanvas` renders the Markdown returned by `/api/workspace/file` (GFM tables, task lists
  and fenced code), and the active path is bound to every co-pilot turn (R1).
- Slice 2 — `POST /api/copilot/chat` streams normalized `token` / `error` / `done` Server-Sent
  Events proxying the Gemini REST API, with typed failure modes (`PROVIDER_NOT_CONFIGURED` 424,
  `UPSTREAM_UNAVAILABLE` 502, `TRAVERSAL_DETECTED` 403, `EMPTY_PROMPT` 400).
- Slice 3 — `GET`/`POST /api/settings/copilot` persists the provider credential in the SQLite
  `settings` table and never returns it to the client; the settings dialog is write-only by
  construction (R3).

The panel consumes the stream off a `ReadableStream` reader rather than blocking on a complete
response, so tokens paint incrementally (R2). A failed turn is rolled back: the transcript keeps
only completed turns, the prompt returns to the composer untouched, and an inline banner offers a
retry (S2).

Scenarios S1, S2, S3, S4 and S5 are covered by
`apps/ade/tests/epic_02_conversational_copilot_panel_tests.rs` against a local stub provider, so
the suite needs neither network access nor a real credential. Full suite: 24 tests green, and
`just check` (SCPE validate, `cargo fmt`, `prettier`, `cargo clippy -D warnings`, `oxlint`,
`tsc --noEmit`) is clean.

Verified end-to-end in the browser against the real binary: tree navigation and canvas rendering,
incremental token streaming, context rebinding on document switch, the `UPSTREAM_UNAVAILABLE`
banner with a working retry, and the write-only provider dialog.

## Blockers
None.

## Log
- 2026-09-23 — Initial creation in Draft state.
- 2026-09-25 — plan.md refined to 5 rules, 5 scenarios and 3 vertical slices; Readiness Gate passed.
- 2026-09-25 — Draft → Ready approved by Tech Lead (m4rc0s).
- 2026-09-25 — Ready → WIP; TASK-02 and TASK-03 delivered and verified.
- 2026-09-25 — TASK-01 and TASK-04 delivered; frontend slice verified in-browser.
- 2026-09-25 — WIP → Done by m4rc0s.
