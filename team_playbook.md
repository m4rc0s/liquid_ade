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

## Branching & Linear History Strategy (Squash Merge)

The project maintains a **100% linear, bifurcation-free history on `main`** using the Squash Merge strategy:
1. **Branch Creation:** Never commit directly to `main` during feature or epic implementation. Always branch from `main`:
   ```bash
   just branch-start <epic-slug>
   # or: git checkout -b feat/<epic-slug>
   ```
2. **Implementation & Iterative Commits:**
   - Implement tasks cited in `tasks.md`.
   - Commit iteratively on the feature branch.
   - Run `just check` (SCPE validate, formatting, clippy, oxlint, and tsc) to ensure zero errors.
3. **Squash Merge into main (Linear History with Curated Commit Description):**
   - Integrate into `main` as a single canonical commit per epic.
   - **Curated Commit Description Requirement:** The commit body must explicitly list the most important commits and changes (key features, architectural/spec updates, and test suites), filtering out trivial or WIP noise:
   ```bash
   git checkout main
   git merge --squash feat/<epic-slug>
   git commit -m "feat(<epic-slug>): <canonical english summary>" -m "Key changes:
   - feat(<scope>): <key feature implementation>
   - docs(<scope>): <spec or architecture update>
   - test(<scope>): <verification tests for epic#S#>"
   git branch -D feat/<epic-slug>
   ```
   - **Result:** `main` remains a perfectly straight line where every commit is rich in context, self-documenting, and free of noisy temporary commits.

## Releases & Changelog Management

- **Changelog SSOT:** Maintained in [`CHANGELOG.md`](CHANGELOG.md) following Keep a Changelog and SemVer 2.0.0.
- **Release Protocol:**
  1. Move items from `## [Unreleased]` to `## [X.Y.Z] - YYYY-MM-DD`.
  2. Synchronize `version = "X.Y.Z"` across `apps/ade/Cargo.toml` and `apps/ade/ui/package.json`.
  3. Commit: `chore(release): bump version to vX.Y.Z`.
  4. Tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z - <Milestone Title>"`.

