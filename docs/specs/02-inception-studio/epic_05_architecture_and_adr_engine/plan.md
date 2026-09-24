# Plan: Architecture Guidelines & ADR Engine

## Intent
Provide conversational co-pilot prompts to define and refine system architecture guidelines and author formal Architectural Decision Records with explicit context, decision, consequences, and rejected alternatives.

## Domain Model
- **Terms:** Architectural Decision Record, Code as Consequence
- **Entities:** AdrDocument, ArchitecturalBoundary, DecisionRecord
- **Domain events:** AdrDrafted, TechnicalDealUpdated

## Business Rules
- **R1:** Every generated ADR must include Status, Context, Decision, Consequences and Tradeoffs, and Rejected Alternatives.
- **R2:** All tool recommendations must be verified against approved runtime constraints and recorded in technical_deal.md.

## Examples
- **S1 — Author new ADR via co-pilot:** Given an active architecture planning session, When the user requests an ADR for selecting Axum over Actix-web, Then the co-pilot formulates an ADR with Context, Decision, Consequences, and Rejected Alternatives and presents it for approval.
- **S2 — Append approved ADR to architecture.md:** Given an approved ADR draft, When the user confirms saving, Then the ADR is appended under `## Decisions` in `architecture.md` on disk.

## Slices
1. Conversational ADR authoring and architecture file updater — S1, S2, R1, R2

## Open Questions
- None. [resolved]
