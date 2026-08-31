# 🌊 Liquid ADE (Agentic Development Environment)

**Single-Binary ADE para Desenvolvimento Orientado a Especificações (SCPE v0.3.0) com Inteligência Artificial**

---

## 🏛️ Arquitetura da Aplicação

A aplicação **Liquid ADE** é construída como um monorepo enxuto de alta performance, projetado para ser distribuído como um **binário único auto-contido**, sem necessidade de runtime Node.js/Bun em produção:

```text
apps/ade/
├── Cargo.toml          # Maestro Rust (Axum, Tokio, rust-embed)
├── build.rs            # Script de compilação que faz o build da UI e a embute no binário
├── app_manifest.md     # Manifesto do aplicativo conforme SCPE v0.3.0
├── src/                # ade_core: Servidor ACP (Agent Client Protocol), LiteLLM Gateway e Router
│   └── main.rs
└── ui/                 # ade_ui: Frontend React SPA (Vite, TypeScript, Tailwind, shadcn/ui, Zustand)
    ├── package.json
    ├── vite.config.ts
    └── src/
```

---

## 🧩 Componentes

1. **`ade_core` (Rust / Axum / Tokio):**
   - Servidor HTTP e WebSocket implementando o protocolo **ACP (Agent Client Protocol)** via JSON-RPC 2.0.
   - Orquestração de modelos de IA via **LiteLLM** (suporte transparente a Ollama, Llama.cpp, Claude, OpenAI, Gemini).
   - Servidor estático dos arquivos da `ade_ui` a partir da memória RAM via `rust-embed`.

2. **`ade_ui` (React / Vite / Tailwind):**
   - Painel visual de engenharia de produto (ADE).
   - **Document-as-UI / Editor WYSIWYG:** Renderização e edição de especificações Markdown em blocos visuais ricos com sincronização bidirecional em tempo real.
   - Navegador de workspace SCPE e auditor de máquina de estados (`Draft → Ready → WIP → Done → Stale`).

---

## 🚀 Como Rodar

### Modo Desenvolvimento
```bash
# Rodar o frontend React (ade_ui) isolado com Hot Reload
cd ui && bun run dev

# Rodar o core em Rust (ade_core)
cargo run
```

### Compilação do Binário Único (Release)
```bash
# Compila a UI e embute automaticamente dentro do executável Rust
cargo build --release
```
