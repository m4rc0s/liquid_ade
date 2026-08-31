# 🌊 Liquid ADE (Agentic Development Environment)

**Single-Binary ADE for [Spec-Compiled Product Engineering (SCPE)](https://github.com/m4rc0s/scpe) with Autonomous AI Agents**

---

## 🏛️ Application Architecture

The **Liquid ADE** application is built as a lean, high-performance monorepo designed to be distributed as a **self-contained single binary**, with zero runtime dependencies on Node.js or Bun in production:

```text
apps/ade/
├── Cargo.toml          # Rust Workspace Master (Axum, Tokio, rust-embed)
├── build.rs            # Build script that compiles the UI and embeds it into the binary
├── app_manifest.md     # Application manifest following SCPE specifications
├── src/                # ade_core: ACP Server (Agent Client Protocol), LiteLLM Gateway & Router
│   └── main.rs
└── ui/                 # ade_ui: Frontend React SPA (Vite, TypeScript, Tailwind, Zustand)
    ├── README.md       # 🔗 Frontend Studio Documentation
    ├── UI_UX_GUIDELINES.md # Astryx Design System Guidelines
    ├── package.json
    ├── vite.config.ts
    └── src/
```

---

## 🧩 Components

1. **`ade_core` (Rust / Axum / Tokio):**
   - HTTP and WebSocket server implementing the **ACP (Agent Client Protocol)** via JSON-RPC 2.0.
   - LLM model orchestration via **LiteLLM** (seamless connectivity to Ollama, Llama.cpp, Claude, OpenAI, Gemini).
   - Embedded static file server for `ade_ui` served directly from RAM using `rust-embed`.

2. **[`ade_ui`](ui/README.md) (React / Vite / Tailwind / Astryx):**
   - High-density Product Engineering visual studio. See [detailed UI documentation](ui/README.md).
   - **Document-as-UI / WYSIWYG Editor:** Real-time rendering and editing of Markdown specifications as rich visual blocks with bidirectional live sync.
   - [SCPE](https://github.com/m4rc0s/scpe) workspace explorer and state machine auditor (`Draft → Ready → WIP → Done → Stale`).

---

## 🚀 Getting Started

### Development Mode
```bash
# Run the React frontend (ade_ui) with Hot Reload
cd ui && bun run dev

# Run the Rust core backend (ade_core)
cargo run
```

### Building the Single Release Binary
```bash
# Automatically builds the UI and embeds it inside the final Rust binary
cargo build --release
```

---

## 📚 Related Documentation

- [📖 Frontend README (`ade_ui`)](ui/README.md)
- [🎨 UI/UX Guidelines (Astryx)](ui/UI_UX_GUIDELINES.md)
- [📋 Application Technical Manifest](app_manifest.md)
- [🌐 Root Product & Workspace README](../../README.md)
- [📐 SCPE Official Methodology Repository](https://github.com/m4rc0s/scpe)
