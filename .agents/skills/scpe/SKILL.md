---
name: scpe
description: >-
  SCPE (Spec-Compiled Product Engineering) method and tooling. Use whenever work happens in an
  SCPE product workspace (a folder with product_vision.md, glossary.md, features/ and apps/), or
  when the user wants to start a new product, write a product vision or glossary, open a feature
  or epic, write or review a plan.md (rules R#, Given/When/Then examples S#, vertical slices),
  run the Readiness Gate, change an epic state (Draft, Ready, WIP, Blocked, Done, Stale), handle
  spec drift, or compile a Ready epic into tasks.md, code and tests in apps/.
---

# SCPE — Spec-Compiled Product Engineering

Living Markdown documentation is the contract; code in `apps/` is its consequence.
**Humans decide, agents execute.** The full method is in `references/SCPE_METHOD.md`;
checklists for each wave are in `references/checklists.md`.

## 0. Find the workspace

The workspace is the folder containing `product_vision.md` and `features/` (or `$SCPE_WORKSPACE`).

```bash
python3 "${CLAUDE_PLUGIN_ROOT}/skills/scpe/scripts/scpe.py" status
```

No workspace yet → create one with `init` (see §5). Your scope is that folder only. Before any
work, read `index.md`, `product_vision.md`, `glossary.md`, `architecture.md` and `technical_deal.md`.

## 1. Workspace layout

```text
<product>/
├── index.md  product_vision.md  roadmap.md  glossary.md  architecture.md
├── technical_deal.md  team_playbook.md  quick_status.md  CLAUDE.md
├── assets/
├── apps/<app>/app.md            # + src/ or repo_pointer.md
└── features/<feature>/
    ├── index.md  feat_roadmap.md  quick_status.md
    └── epics/<epic>/
        ├── index.md  plan.md  tasks.md  quick_status.md  epic_roadmap.md
```

Formats are in `references/SCPE_METHOD.md` §5; the scaffolds in `templates/` follow them.
Short fields always use `- **field:** value`.

## 2. Wave pipeline

| Wave | Who | You do |
|---|---|---|
| **Upstream** | Developers + agent as co-pilot | Draft vision, glossary terms, feature `index.md`, epic `plan.md`. Ask; never settle business questions yourself. Epic stays `Draft`. |
| **Readiness Gate** | Tech Lead (a human) | Run `gate-check`, present results, wait for explicit human approval, then record `Ready` with `--by <that person>`. |
| **Downstream** | Agent, reviewed by Developers | Only on `Ready` epics: `WIP` → `tasks.md` → code and tests slice by slice → propose `Done`. |
| **Audit** | Everyone | Log learnings and blockers in `quick_status.md`; refresh the product panel with `status --write`. |

Incremental Feature Discovery: open only the most critical feature now; do not map the whole system.
Vertical slicing is the default: each slice cuts through every layer and gives a user something real.

## 3. Epic states

Exactly one `- **state:**` line in the epic `quick_status.md`. Allowed transitions (enforced by `set-state`):

| From | To | Set by |
|---|---|---|
| `Draft` | `Ready` | Tech Lead, after the gate |
| `Ready` | `WIP` / `Blocked` | Agent / Agent or Developer |
| `WIP` | `Done` / `Blocked` | Agent, after Developer review / Agent or Developer |
| `Blocked` | `Ready` / `Draft` | whoever resolves it (Draft when the model must be reworked) |
| `Done` | `Stale` | automatic on any edit to `plan.md` (plugin hook) |
| `Stale` | `Ready` / `Done` | Tech Lead: Re-execute / Accept the drift (with a note) |

**Never** set `Ready` yourself and never approve your own work. Always change state through
`set-state` so the log and the feature index stay in sync.

## 4. Downstream protocol

1. Confirm the epic is `Ready`; `set-state <f>/<e> WIP --by agent`.
2. Write `tasks.md` from `plan.md`: `- [ ] T1 — description (S1, R2)`. Every task cites `S#`/`R#`;
   every example is covered by a task.
3. Build slice by slice in `apps/<app>/` for the apps listed in the epic `index.md`, following
   `app.md` boundaries and `technical_deal.md` standards. Each slice runs before the next starts.
4. Name at least one test per example `<epic>#S#` (e.g. `pix-payment#S2 expired charge can be recreated`).
5. Log dated learnings in `quick_status.md`. Blocked → `set-state … Blocked` and fill `## Blockers`.
6. **Immediate Task Completion (Strict Invariant):**
   - **Always mark tasks as done (`- [x]`) immediately upon completing and verifying each task.**
   - Never proceed to the next task or finish an execution step leaving completed tasks unmarked (`- [ ]`).
   - Use `python3 "$S" complete-task <feature>/<epic> <task_id_or_number>` or edit `tasks.md` directly.
   - When all tasks are checked off, ask a Developer to review, then `set-state … Done`.
7. Commits: `docs:` for specification, `feat:`/`fix:`/`test:` for code — never mixed.

Found a gap in the plan while building? Stop and ask. Do not silently change `plan.md` or code around it.

## 5. Scripts

`S="${CLAUDE_PLUGIN_ROOT}/skills/scpe/scripts/scpe.py"` — Python 3 standard library only.

| Action | Command |
|---|---|
| New product workspace | `python3 "$S" init <name> [--path DIR] [--app NAME:TYPE]... [--no-git]` |
| New feature | `python3 "$S" feature <slug> "<Title>"` |
| New epic (Draft) | `python3 "$S" epic <feature> <slug> "<Title>"` |
| Complete task in epic | `python3 "$S" complete-task <feature>/<epic> <task_id_or_number>` |
| Readiness Gate lint | `python3 "$S" gate-check <feature>/<epic>` |
| Change state | `python3 "$S" set-state <feature>/<epic> <State> --by "<who>" [--note "..."]` |
| Validate workspace | `python3 "$S" validate [--strict]` |
| Status panel | `python3 "$S" status [--json] [--write]` |

Scaffolds never overwrite: they abort if the target exists. `validate --strict` also requires a
`<epic>#S#` test for every example of a `Done` epic and turns warnings into errors.

## 6. Writing a good plan.md

- One `When` per example; `Then` is observable by a user or another system.
- If you cannot write the example, the decision has not been made — add an Open Question.
- Open questions block the gate unless marked `(non-blocking)` or `[resolved]`.
- Terms in **Domain Model** must exist as `##` headings in `glossary.md`.
- Slices list the `S#`/`R#` they deliver; if not vertical, say why.
