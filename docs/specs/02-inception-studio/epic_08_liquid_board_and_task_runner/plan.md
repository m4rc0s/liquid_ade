# Plan: Liquid Board & One-Click Task Runner

## Intent
Provide a Jira-style visual Kanban board projecting the SCPE lifecycle states with epic cards and task checklists, alongside an interactive one-click action runner to trigger agent task execution and automatically update disk state.

## Domain Model
- **Terms:** Liquid Board, One-Click Task Runner
- **Entities:** KanbanBoard, EpicCard, TaskItemTrigger, AgentExecutionSession
- **Domain events:** TaskExecutionTriggered, TaskCompletedOnDisk, BoardStateRefreshed

## Business Rules
- **R1:** Clicking execute on a task must load the task context, parent epic plan, rules R#, and acceptance criteria before prompting the agent.
- **R2:** On successful completion, tasks.md must be updated on disk to `- [x]` with echo suppression, and quick_status.md must be refreshed.

## Examples
- **S1 — 1-click execution triggers agent:** Given an epic with unchecked task `- [ ] TASK-01 (S1, R1)`, When the user clicks the execute button `[▶]`, Then the execution drawer opens with real-time streaming activity from the agent.
- **S2 — Task completion marks checkbox on disk:** Given the agent finishes executing the task successfully, When the execution terminates, Then `tasks.md` on disk contains `- [x] TASK-01 (S1, R1)` and the UI card updates progress.

## Slices
1. Jira-style Kanban board view and one-click task execution runner — S1, S2, R1, R2

## Open Questions
- None. [resolved]
