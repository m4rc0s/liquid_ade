# Team Playbook: {{product_name}}

## Roles
- **Developers** — everyone who shapes the product (PM, design, engineering, domain experts). They model intent in Upstream.
- **Tech Lead** — runs the Readiness Gate and decides on spec drift.
- **Agents** — draft documents, write `tasks.md`, write code and tests. Never approve.

## Wave Pipeline
1. **Upstream** — scope the feature and epic; model `plan.md` (terms, rules, examples, slices).
2. **Readiness Gate** — the Tech Lead validates the model and marks the epic `Ready`.
3. **Downstream** — the agent writes `tasks.md`, then code and tests, slice by slice.
4. **Audit** — everyone tracks progress, learnings and blockers in `quick_status.md`.

Waves run concurrently across features.

## Rules of Engagement
<!-- Cadence, review expectations, how disagreements are settled, who the Tech Lead is. -->

## Commits
- `docs:` for specification changes, `feat:`/`fix:`/`test:`/`refactor:` for code. Never mix both in one commit.
