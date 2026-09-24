# Plan: Liquid Board & One-Click Task Runner

## Intent
Provide a Jira-style visual Kanban board projecting the SCPE lifecycle states with epic cards and task checklists, alongside an interactive one-click action runner directly on tasks.md (via Action Pill `▶ Start task` → `🔄 Task in progress` → `✅ Task completed [View changes] [View execution]`) with steering document injection, dual-ledger durable state tracking, and deterministic acceptance condition verification before disk sync.

## Domain Model
- **Terms:** Liquid Board, One-Click Task Runner, Dual-Ledger Session, Steering Documents, Execution Checkpoint
- **Entities:** KanbanBoard, EpicCard, ActionableTaskBlock, ExecutionDrawer, SteeringContext, WorkLedgerRecord, AcceptanceEvaluator
- **Domain events:** TaskExecutionStarted, SteeringContextInjected, ToolStepStreamed, WorkerCompletionClaimed, AcceptanceVerdictEvaluated, TaskCompletedOnDisk, CheckpointPersisted

## Business Rules
- **R1:** Triggering execution via the Action Pill (`▶ Start task`) must bind the task to an execution session, inject steering documents (`product_vision.md`, `architecture.md`, `technical_deal.md`, and the active epic `plan.md`), and transition the Action Pill to `🔄 Task in progress`.
- **R2:** The right execution drawer must render the streaming feed with specialized action cards: agent thoughts, file reads (`Read [file]`), file diffs (`Accepted edits [View diff]`), static diagnostics (`Checked diagnostics`), command execution with terminal output, status badges (`Status: In Progress` → `Completed`), and summary markdown.
- **R3:** All execution state transitions (`Planning`, `InProgress`, `Verification`, `Completed`, `Failed`) and checkpoints (`last_task_id`, restore point) must be recorded in the SQLite state ledger so long-running sessions outlast conversation memory.
- **R4:** A worker's completion claim (`status = done`) must never mark the task finished unconditionally; the engine must run automated acceptance verification (evaluating file presence, clean diagnostics, test pass) before emitting `verdict = pass`.
- **R5:** On verified completion, `tasks.md` on disk must be atomically patched to `- [x]`, the Action Pill must render `✅ Task completed [View changes] [View execution]`, and the quick status matrix must update progress.

## Examples
- **S1 — 1-click execution launches session with streaming cards:** Given `tasks.md` with `- [ ] TASK-01 (S1, R1)`, When the user clicks the Action Pill `▶ Start task`, Then the pill becomes `🔄 Task in progress` and the right execution drawer displays steering document pills (`product_vision.md`, `architecture.md`, `plan.md`) with streaming tool action cards (`Read`, `Edit with diff`, `Diagnostics`, `Command`).
- **S2 — Worker completion verified and written to disk:** Given an active task session claiming completion, When the acceptance evaluator verifies that the code builds and tests pass, Then `tasks.md` is updated on disk with `- [x] TASK-01`, and the Action Pill displays `✅ Task completed [View changes] [View execution]`.
- **S3 — Session resumption from SQLite checkpoint:** Given an interrupted task execution, When the session is resumed or reopened, Then the runner reads the SQLite ledger to identify `last_task_id` and resume exact progress without re-evaluating completed steps.

## Slices
1. Jira-style Liquid Board projection and Action Pill trigger — S1, R1
2. Dual-ledger session execution drawer with steering context and streaming action cards — S1, S3, R1, R2, R3
3. Acceptance verification evaluator and atomic disk update (`- [x]`) — S2, R4, R5

## Open Questions
- None. [resolved]
