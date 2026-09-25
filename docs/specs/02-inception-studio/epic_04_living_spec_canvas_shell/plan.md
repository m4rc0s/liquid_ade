# Plan: Living Spec Canvas Shell & Spec-First Navigation

## Intent
Restore and elevate the product-focused UX of Liquid ADE by replacing the generic file-tree-first layout with a canonical 48px Mode Rail and Spec Navigator, projecting living specifications as structured Astryx cards on the Living Spec Canvas with Phase Pills and Action Pills, and hosting the Spec Inspector and Conversational Co-Pilot in a unified right-side tabbed context column.

## Domain Model
- **Terms:** Mode Rail, Spec Navigator, Living Spec Canvas, Phase Pill, Action Pill, Conversational Co-Pilot, Document-as-UI
- **Entities:** ModeRailNavigator, SpecOutlineTree, EpicDetailCardModel, PhaseFilterState, ActionPillDispatcher, SessionCardTranscript
- **Domain events:** ModeSwitched, EpicSelected, PhasePillToggled, ActionPillTriggered, ContextTabSwitched, CardAppended

## Business Rules
- **R1:** The navigator panel is selected by the Mode Rail; exactly one mode is active, `Specs` is the boot default, and the filesystem tree is reachable only by explicitly selecting `Files`. Modes whose epic has not landed are not rendered at all.
- **R2:** Every epic row carries its lifecycle state and task progress read from disk, never a client cache. A missing, duplicated or unrecognised state renders as its raw text, never guessed.
- **R3:** The canvas projects a selected epic as structured cards — Intent, Domain Model, Rules, Examples, Slices, Open Questions, Tasks — never as undifferentiated Markdown. A plain file keeps rendering through the existing Markdown renderer.
- **R4:** The right column hosts exactly one of Inspector or Co-Pilot at a time; switching must not drop an in-flight stream.
- **R5:** Every SCPE read resolves through the existing path guard (`workspace.rs:29`); an escaping, malformed or absent reference is a typed JSON error, never a panic or opaque 500.
- **R6:** An Action Pill renders only states the running binary can reach. Here that is exactly one: dispatching the task to the Conversational Co-Pilot. The canvas never writes to `tasks.md` — the execution lifecycle and disk write-back belong to `epic_10`.
- **R7:** The session transcript is an ordered list of typed cards dispatched on a discriminated `kind` (`prompt`, `answer`, `failure`), not a message array plus a streaming buffer.

## Examples
- **S1 — Workspace outline projection:** Given a workspace containing canonical features and epics on disk, When a client requests `GET /api/scpe/outline`, Then the server responds 200 with all features and epics projecting their lifecycle state, task progress (`tasks_done`/`tasks_total`), and canonical document paths with leading slashes.
- **S2 — Epic detail card projection:** Given a valid feature and epic slug, When a client requests `GET /api/scpe/epic?feature=02-inception-studio&epic=epic_02_conversational_copilot_panel`, Then the server responds 200 with parsed Intent, rules R1–R5, examples S1–S5, slices citing their R#/S# IDs, and raw section bodies.
- **S3 — Path traversal protection on spec reader:** Given a client request containing an escaping path parameter, When a GET request is sent to `/api/scpe/epic?feature=../../../etc&epic=passwd`, Then the server responds 403 with error code `TRAVERSAL_DETECTED` and reads no files outside the workspace root.
- **S4 — State machine casing and round-trip fidelity:** Given epics on disk with various SCPE lifecycle states, When the outline endpoint reads each epic's `quick_status.md`, Then all six states round-trip exactly including uppercase `WIP` and unrecognised values as raw text without silent coercion to Draft.
- **S5 — Task list with badge metadata parsing:** Given a task line in `tasks.md` formatted as `- [x] TASK-02: Stream tokens (S1, S2, S5, R2, R4, R5)`, When the epic detail endpoint parses the file, Then the returned task object contains `done: true`, `label: "TASK-02"`, `examples: ["S1", "S2", "S5"]`, and `rules: ["R2", "R4", "R5"]`.
- **S6 — Co-pilot document binding to active epic plan:** Given an epic selected on the Spec Navigator without a specific document file selected, When the user submits a prompt in the Conversational Co-Pilot tab, Then the turn automatically binds that epic's `plan.md` path as the active context document.

## Slices
1. Spec Navigator over the Mode Rail: backend `/api/scpe/outline` and frontend `ModeRail` / `SpecNavigator` — S1, S4, R1, R2, R5
2. Living Spec Canvas with Phase Pills: backend `/api/scpe/epic` and frontend `SpecCanvas`, `PhasePills`, and `SpecCards` — S2, S3, S5, R3, R5
3. Action Pill dispatch + right-column tabs: tabbed Inspector/Co-Pilot right column and Action Pill task dispatch — S6, R4, R6
4. Session console card shell: discriminated `SessionCard` transcript list replacing legacy string buffer — R7

Each slice is vertical: slice 1 spans outline disk parsing to active navigation rail; slice 2 spans detail parsing to interactive card rendering; slice 3 spans task click to context-bound co-pilot turn; slice 4 spans event stream to typed card projection.

## Open Questions
- Outline cost ceiling on large workspaces: ~20 file reads on this workspace, ~600 on a 200-epic workspace. Day 1 performs direct disk reads; in-memory caching and reactive invalidation are scheduled for Phase 3 via the `notify` file watcher. (non-blocking)
