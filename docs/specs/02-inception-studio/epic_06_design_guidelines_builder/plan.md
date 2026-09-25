# Plan: Design Guidelines Builder & UI/UX Standards Formulator

## Intent
Provide guided dialogue and automated document creation for `UI_UX_GUIDELINES.md` based on the Astryx Design System, allowing the user to configure typography, surface palettes, density, dark theme rules, and UX heuristics.

## Domain Model
- **Terms:** Design Guidelines, Document-as-UI
- **Entities:** DesignTokenSet, ThemeConfig, GuidelineDocument
- **Domain events:** GuidelinesFormulated, ThemeTokensCommitted

## Business Rules
- **R1:** Generated guidelines must conform to the Astryx Design System token taxonomy (surfaces, typography, spacing, states).
- **R2:** All guideline changes must be written directly to `UI_UX_GUIDELINES.md` in the target app or workspace root.

## Examples
- **S1 — Formulate design guidelines with co-pilot:** Given an active inception session, When the user triggers "Define UI/UX Guidelines", Then the co-pilot guides the user through palette and density choices and generates a complete `UI_UX_GUIDELINES.md`.
- **S2 — Validate theme contrast heuristic:** Given high-density mode is selected, When dark theme rules are generated, Then the guidelines specify contrast ratios conforming to WCAG AA.

## Slices
1. Astryx design guidelines generator and file writer — S1, S2, R1, R2

## Open Questions
- None. [resolved]
