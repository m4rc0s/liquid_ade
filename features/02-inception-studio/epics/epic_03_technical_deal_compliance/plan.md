# Plan: Technical Deal Compliance & Astryx Foundation

## Intent
Bring the application into strict compliance with `technical_deal.md`, resolving all active stack divergences between declared architecture and downstream code. Upgrade Axum to 0.8, adopt the official `@astryxdesign/core` component library alongside Tailwind CSS overrides, replace ad-hoc `useState` clustering with Zustand stores, enable TypeScript `strict` mode in `tsconfig.app.json`, complete the SCPE design tokens (`--status-stale` and chip modifiers for all 6 SCPE states: Draft, Ready, WIP, Blocked, Done, Stale), and extract the visual specification tokens from PR #1 CSS into reusable Astryx domain components (`SpecCard`, `TaskItem`, `RuleItem`, `StateChip`, `Chip`) so that downstream epics compose a unified design system vocabulary.

## Domain Model
- **Terms:** Design Guidelines, Pure Static SPA, SSOT, Code as Consequence
- **Entities:** TechnicalStackCompliance, AstryxDesignSystem, GlobalWorkspaceStore, DesignTokenRegistry
- **Domain events:** AxumFrameworkUpgraded, AstryxCoreAdopted, ZustandStoreExtracted, TypeScriptStrictEnforced, DesignTokensAligned

## Business Rules
- **R1:** The backend HTTP and routing framework must run on Axum 0.8 as declared in `technical_deal.md:10`, with all existing routes (`/api/health`, `/api/workspace/*`, `/api/projects/*`, `/api/copilot/*`, `/api/settings/*`) compiling and passing tests without deprecation warnings.
- **R2:** Frontend design system must integrate official `@astryxdesign/core` (Meta, MIT) with Tailwind CSS styling overrides; raw monolithic `.astryx-*` rules in `App.css` must be replaced with modular, typed Astryx component wrappers (`SpecCard`, `TaskItem`, `RuleItem`, `StateChip`, `Chip`).
- **R3:** The design token palette in `index.css` must provide complete semantic tokens for all six SCPE lifecycle states (`--status-draft`, `--status-ready`, `--status-wip`, `--status-blocked`, `--status-done`, `--status-stale`), and `.astryx-chip` must provide matching modifier classes for all six states.
- **R4:** Frontend global and cross-panel state must be managed via decoupled Zustand stores (`apps/ade/ui/src/stores/`) rather than prop drilling or monolithic `useState` clusters in `App.tsx`.
- **R5:** TypeScript compilation must enforce `"strict": true` in `tsconfig.app.json` (per `technical_deal.md:50`) with zero type errors, zero implicit `any`, and strict null checks across all UI source files.

## Examples
- **S1 — Axum 0.8 framework compliance:** Given the ADE application compiled with Axum 0.8, When the test suite executes all health, workspace, and copilot endpoints, Then all existing tests pass with 200/403/424 responses and zero compiler warnings.
- **S2 — Complete SCPE state token coverage:** Given the design system stylesheets in `index.css` and `App.css`, When checking color variables and `.astryx-chip` classes, Then tokens and classes exist for all six SCPE states including `stale` and `blocked`.
- **S3 — Astryx component encapsulation:** Given the extracted Astryx component library in `apps/ade/ui/src/components/`, When rendering `StateChip` with state `"WIP"` and `TaskItem` with badges `"S1, R1"`, Then the components render valid semantic markup with Astryx styling without relying on dead PR #1 CSS rules.
- **S4 — Zustand workspace state migration:** Given the application UI booting in the browser, When the active project or opened document path changes, Then the update is dispatched through a Zustand store and subscribed components re-render reactively.
- **S5 — TypeScript strict mode validation:** Given `tsconfig.app.json` configured with `"strict": true`, When `bunx tsc -b --noEmit` runs, Then type-checking completes with exit code 0 and 0 errors.

## Slices
1. Backend stack compliance: Axum 0.8 upgrade and route verification — S1, R1
2. Design token completion and Astryx core component extraction (`SpecCard`, `TaskItem`, `RuleItem`, `StateChip`, `Chip`) — S2, S3, R2, R3
3. Frontend architecture compliance: Zustand store extraction and TypeScript strict mode enforcement — S4, S5, R4, R5

Each slice is vertical: slice 1 verifies backend dependency to HTTP response; slice 2 verifies CSS tokens to rendered component DOM; slice 3 verifies state action to UI reactivity under strict compilation.

## Open Questions
- `@astryxdesign/core` NPM availability: verified published as v0.6.3 (MIT) from Meta, React 19 peer dependency. [resolved]
- Frontend test runner adoption: declared as `vitest` in `technical_deal.md`, implementation sequenced in dedicated later epic. (non-blocking)
