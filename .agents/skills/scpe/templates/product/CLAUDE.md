# {{product_name}} — SCPE Product Workspace

This folder is a product workspace that follows **SCPE (Spec-Compiled Product Engineering)**.
The Markdown specification is the contract; code in `apps/` is its consequence.

## Rules for agents

- Your scope is this folder only. Read `index.md` first, then `product_vision.md`,
  `glossary.md`, `architecture.md` and `technical_deal.md` before any work.
- Use the exact terms in `glossary.md`. Propose new terms; never invent synonyms.
- Epic state lives in `features/<f>/epics/<e>/quick_status.md` and is one of
  `Draft`, `Ready`, `WIP`, `Blocked`, `Done`, `Stale`.
- Only a human acting as Tech Lead marks an epic `Ready`. Never approve your own work.
- Build only `Ready` epics: set `WIP`, write `tasks.md` citing `R#`/`S#`, write code in
  `apps/<app>/` within its `app.md`, name tests `<epic>#S#`, then propose `Done` for review.
- Editing the `plan.md` of a `Done` epic makes it `Stale`.
- Ask the Developers about open business questions instead of guessing.

If the `scpe` Claude Code plugin is installed, use `/scpe-status`, `/scpe-validate`,
`/scpe-feature`, `/scpe-epic` and `/scpe-gate`.
