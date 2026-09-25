# Plan: Conversational Co-Pilot Panel & Streaming Client

## Intent
Provide a dual-pane conversational co-pilot panel where the user interacts with an AI agent in a chat pane while simultaneously viewing and focusing on the relevant specification document canvas.

## Domain Model
- **Terms:** Conversational Co-Pilot, Document-as-UI
- **Entities:** CopilotSession, ChatMessage, CanvasDocumentBinding
- **Domain events:** PromptSubmitted, TokenStreamReceived, DocumentContextBound

## Business Rules
- **R1:** The co-pilot panel must maintain document context binding so every conversation turn knows the active file on the document canvas.
- **R2:** Responses from the model must stream incrementally without blocking the user interface main thread.

## Examples
- **S1 — Streaming co-pilot message:** Given a user opens the co-pilot panel with `product_vision.md` active on the canvas, When the user sends prompt "Clarify our target audience", Then tokens stream into the chat panel and the active document context header displays `product_vision.md`.
- **S2 — Resilient error handling on model failure:** Given an offline inference backend, When the user submits a message, Then the chat panel displays an inline error banner with a retry action without clearing the input.

## Slices
1. Dual-pane UI layout and streaming token chat client — S1, S2, R1, R2

## Open Questions
- None. [resolved]
