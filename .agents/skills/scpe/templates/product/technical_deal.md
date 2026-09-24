# Technical Deal: {{product_name}}

Technical agreements every Developer and agent follows. Code in `apps/` is generated
against this file.

## Approved Stack
<!-- Languages, frameworks, datastores, infrastructure. -->

## Engineering Standards
<!-- e.g. SOLID, Clean Architecture, Hexagonal (Ports and Adapters), established design patterns. -->

## AI Guardrails
- Humans decide, agents execute. No agent approves its own work or settles an open business question.
- Agents build only epics in state `Ready`, and only the scope of their `plan.md`.
- Every task in `tasks.md` cites the rules (`R#`) or examples (`S#`) it implements.
- Every example has at least one test named `<epic_name>#S#`.
- Code lives only in `apps/<app>/`, inside the boundaries of its `app.md`.
- Any edit to the `plan.md` of a `Done` epic makes it `Stale`.

## Constraints
<!-- Compliance, security, performance, budget, supported platforms. -->

## Definition of Done
- Tasks in `tasks.md` all checked.
- Tests for every example pass.
- Code reviewed by a Developer.
- `quick_status.md` log updated with learnings.
