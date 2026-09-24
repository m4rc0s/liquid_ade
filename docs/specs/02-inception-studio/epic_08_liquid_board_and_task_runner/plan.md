# Plan: Liquid Board & One-Click Task Runner

## Intent
Provide a Jira-style visual Kanban board projecting the SCPE lifecycle states with epic cards and task checklists, alongside an interactive one-click action runner directly on tasks.md (`▶ Start task` → `🔄 Task in progress` → `✅ Task completed [View changes] [View execution]`) with steering document injection, dual-ledger durable state tracking, and deterministic acceptance condition verification before disk sync.

## Domain Model
- **Terms:** Liquid Board, One-Click Task Runner, Dual-Ledger Session, Steering Documents, Execution Checkpoint
- **Entities:** KanbanBoard, EpicCard, ActionableTaskBlock, ExecutionDrawer, SteeringContext, WorkLedgerRecord, AcceptanceEvaluator
- **Domain events:** TaskExecutionStarted, SteeringContextInjected, ToolStepStreamed, WorkerCompletionClaimed, AcceptanceVerdictEvaluated, TaskCompletedOnDisk, CheckpointPersisted

## Business Rules
- **R1:** Triggering execution (`▶ Start task`) must bind the task to an execution session, inject steering documents (`product_vision.md`, `architecture.md`, `technical_deal.md`, and the active epic `plan.md`), and transition the UI state to `🔄 Task in progress`.
- **R2:** All execution state transitions (`Planning`, `InProgress`, `Verification`, `Completed`, `Failed`) and checkpoints (`last_task_id`, restore point) must be recorded in the SQLite state ledger so long-running sessions outlast conversation memory.
- **R3:** A worker's completion claim (`status = done`) must never mark the task finished unconditionally; the engine must run automated acceptance verification (evaluating file presence, clean diagnostics, test pass) before emitting `verdict = pass`.
- **R4:** On verified completion, `tasks.md` on disk must be atomically patched to `- [x]`, the task block must render `✅ Task completed [View changes] [View execution]`, and the quick status matrix must update progress.

## Examples
- **S1 — 1-click execution launches session with steering documents:** Given `tasks.md` with `- [ ] TASK-01 (S1, R1)`, When the user clicks `▶ Start task`, Then the button becomes `🔄 Task in progress` and the execution drawer displays steering document pills (`product_vision.md`, `architecture.md`, `plan.md`) with streaming tool actions (`Read`, `Edit`, `Diagnostics`).
- **S2 — Worker completion verified and written to disk:** Given an active task session claiming completion, When the acceptance evaluator verifies that the code builds and tests pass, Then `tasks.md` is updated on disk with `- [x] TASK-01`, and the UI displays `✅ Task completed [View changes] [View execution]`.
- **S3 — Session resumption from SQLite checkpoint:** Given an interrupted task execution, When the session is resumed or reopened, Then the runner reads the SQLite ledger to identify `last_task_id` and resume exact progress without re-evaluating completed steps.

## Slices
1. Jira-style Liquid Board projection and actionable task block trigger — S1, R1
2. Dual-ledger session execution drawer with steering context and tool streaming — S1, S3, R1, R2
3. Acceptance verification evaluator and atomic disk update (`- [x]`) — S2, R3, R4

## Open Questions
- None. [resolved]
