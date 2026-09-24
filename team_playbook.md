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

## Branching, PR Workflow & Linear History Strategy (Option B)

The project maintains a **100% linear, bifurcation-free history on `main`** governed by a GitHub Pull Request and remote Squash Merge workflow:

1. **Branch Creation:**
   Never develop directly on `main`. Always branch from the latest `main`:
   ```bash
   git checkout main && git pull origin main
   just branch-start <epic-slug>
   # or: git checkout -b feat/<epic-slug>
   ```

2. **Implementation, Testing & Verification:**
   - Implement tasks cited in `tasks.md`.
   - Commit iteratively on the feature branch.
   - Run `just check` and `just test` (SCPE validate, formatting, clippy, oxlint, tsc, and cargo test) to ensure zero errors and zero warnings.

3. **Push Branch & Open Pull Request:**
   - Push the branch to remote:
     ```bash
     git push -u origin feat/<epic-slug>
     ```
   - Open a Pull Request using `gh pr create` or the GitHub UI targeting `main`:
     - **Title:** `<type>(<scope>): <concise description in English>`
     - **Body:** Must include a curated summary of key changes, architectural impacts, and BDD verification scenarios (`<epic>#S#`).

4. **Human Review & Remote Squash Merge:**
   - The Tech Lead reviews the PR on GitHub.
   - Merge is executed on GitHub using **Squash and Merge** (or `gh pr merge --squash`), keeping the curated commit description and ensuring `main` remains a clean, linear sequence of milestone commits.

5. **Local Synchronization & Cleanup:**
   - After the remote merge, synchronize the local repository:
     ```bash
     git checkout main
     git pull origin main
     git branch -d feat/<epic-slug>
     ```

## Releases & Changelog Management

- **Changelog SSOT:** Maintained in [`CHANGELOG.md`](CHANGELOG.md) following Keep a Changelog and SemVer 2.0.0.
- **Release Protocol:**
  1. Move items from `## [Unreleased]` to `## [X.Y.Z] - YYYY-MM-DD`.
  2. Synchronize `version = "X.Y.Z"` across `apps/ade/Cargo.toml` and `apps/ade/ui/package.json`.
  3. Commit: `chore(release): bump version to vX.Y.Z`.
  4. Tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z - <Milestone Title>"`.

