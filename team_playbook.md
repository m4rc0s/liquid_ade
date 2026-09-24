# Team Playbook: Liquid ADE

## Roles

- **Developers / Architects** — Senior Architects, Product Engineers, and domain experts shaping the product. They model intent in Upstream, formulate ADRs, and supervise technical boundaries.
- **Tech Lead** — Human lead who runs the Readiness Gate, decides on spec drift, and authorizes downstream builds.
- **Agents** — `architect` and specialized worker agents that draft documents, formulate task breakdowns, compile code, and write automated tests. Agents never unilaterally approve their own work.

## Wave Pipeline

1. **Upstream (The Rising Wave)** — Inception and specification. Scope the feature and epic; model `plan.md` (terms, rules `R#`, examples `S#`, slices).
2. **Readiness Gate** — The Tech Lead validates the model against acceptance criteria and transitions the epic from `Draft` to `Ready`.
3. **Downstream (The Breaking Wave)** — Autonomous agents compile `tasks.md`, implement code in Podman sandbox, write verification tests (`<epic>#S#`), and update task states.
4. **Audit & Verification** — Track progress, test executions, learnings, and blockers in `quick_status.md`.

Waves run concurrently across features.

## Rules of Engagement

- Specifications precede code. Divergence between spec and code is an error to resolve in the spec.
- Small, focused epics: Epics are scoped narrowly to fit within minimal agent context windows.
- Immediate Task Update: Mark tasks `- [x]` immediately upon completion.
- English Git Commits: Commit messages must strictly follow Conventional Commits in English.

## Commits

- `docs:` for specification changes, `feat:`/`fix:`/`test:`/`refactor:` for code. Never mix both in one commit.
