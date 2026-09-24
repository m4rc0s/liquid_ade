# SCPE Checklists

## Upstream — ready to submit to the gate

- [ ] Feature `index.md` has goal, users, bounded_context, business_value, scope and out of scope.
- [ ] Epic scope is the smallest increment a user can actually use.
- [ ] `plan.md` Intent says who gets what value.
- [ ] Every term in Domain Model is a `##` heading in `glossary.md`.
- [ ] Rules are numbered `R1, R2…`; each is an invariant, not a task.
- [ ] Examples are numbered `S1, S2…`; each has Given, exactly one When, and an observable Then.
- [ ] Slices cite the `S#`/`R#` they deliver; each slice is vertical or the reason is written.
- [ ] Open questions are resolved, or marked `(non-blocking)` with an owner.

## Readiness Gate — Tech Lead

- [ ] `gate-check` passes (no FAIL).
- [ ] The model fits `architecture.md` and the stack in `technical_deal.md`.
- [ ] Every Then is observable by a user or another system.
- [ ] The epic lists the `apps:` it touches, and each has an `app.md`.
- [ ] Decision recorded: `set-state <f>/<e> Ready --by "<Tech Lead>"`.

## Downstream — agent

- [ ] Epic is `Ready`; set `WIP`.
- [ ] Write `tasks.md`: atomic tasks, one per vertical step, each citing `S#`/`R#`; every `S#` covered.
- [ ] Build slice by slice; each slice runs and is demoable before the next starts.
- [ ] Code only in `apps/<app>/`, inside the boundaries of its `app.md`, following `technical_deal.md`.
- [ ] One test per example at least, named `<epic>#S#`.
- [ ] Log learnings and non-plan surprises in `quick_status.md`.
- [ ] Blocked by a dependency or a defect → set `Blocked` and fill `## Blockers`.
- [ ] All tasks checked, tests green, Developer review done → set `Done`.

## Spec drift — Tech Lead (epic is `Stale`)

- [ ] Review only the difference in `plan.md` (`git diff`).
- [ ] **Re-execute** (behavior changed) → `set-state … Ready`, then a new Downstream cycle.
- [ ] **Accept the drift** (cosmetic) → `set-state … Done` with a note.
- [ ] Record the decision in `epic_roadmap.md` under Drift Decisions.
