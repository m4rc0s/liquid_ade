# 🌊 Liquid ADE: Guia Mestre do Workspace

**Metodologia:** SCPE v0.3.0 (Spec-Compiled Product Engineering)  
**Status do Workspace:** Ativo / Fonte Canônica da Verdade (SSOT)  
**Última Atualização:** 2026-08-31  

---

## 1. Visão Geral do Workspace

O workspace **Liquid** é governado estritamente pela metodologia **SCPE v0.3.0**. O repositório centraliza toda a especificação de produto viva na raiz e nas pastas de features, enquanto a implementação técnica reside em `apps/ade/`:

```text
/home/mraraujo/product_design/liquid_ade/
├── 📜 product_vision.md          # Visão do Produto, Manifesto & Pilares Arquiteturais
├── 🗺️ roadmap.md                 # Planejamento dos 6 Cortes Verticais
├── 📖 glossary.md                # Linguagem Ubíqua e Máquina de Estados
├── 🧭 index.md                   # Este Guia Mestre
│
├── 📦 apps/
│   └── ade/                      # Monorepo Rust (Axum/Tokio) + React (Astryx SPA)
│       ├── app_manifest.md       # Manifesto técnico do aplicativo
│       ├── Cargo.toml            # Maestro do Backend
│       ├── build.rs              # Script que embute a UI no binário
│       └── ui/
│           ├── UI_UX_GUIDELINES.md # Constituição com os 7 Pilares de UI e 8 Heurísticas
│           └── package.json      # React 19 + Astryx Design System
│
├── 📂 features/                  # Os 6 Cortes Verticais de Negócio
│   ├── 01-workspace-inspector/   # Visualizador de projetos e leitura de Markdown
│   ├── 02-spec-status-and-drift/ # Badges táteis e detecção de Spec Drift
│   ├── 03-live-file-sync/        # Sincronização em tempo real (<50ms) via Watcher
│   ├── 04-wysiwyg-document-editor/ # Editor WYSIWYG de blocos com patch cirúrgico
│   ├── 05-acp-agent-handshake/   # Protocolo ACP e roteamento LiteLLM
│   └── 06-podman-sandbox-execution/ # Execução de agentes em container Podman
│
└── 🏛️ archive/
    └── v0.5-prototype-legacy/    # Protótipo antigo preservado como histórico
```

---

## 2. Matriz dos Cortes Verticais (`features/`)

Cada pasta dentro de `features/` é um **Bounded Context autocontido** estruturado com:
- `index.md` (PRD da feature e jornada do usuário de ponta a ponta).
- `feat_roadmap.md` (Acompanhamento dos épicos internos).
- `quick_status.md` (Estado formal: `Draft`, `Ready`, `WIP`, `Done`, `Blocked`, `Stale`).
- Subpastas `epic_01_*/` e `epic_02_*/` contendo seus respectivos `index.md`, `plan.md`, `tasks.md` e `quick_status.md`.
