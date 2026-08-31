<div align="center">

# 🌊 Liquid ADE
### *The Spec-First, Agentic Development Environment*

**Transform living Markdown specifications into deterministic, production-ready software.**  
*Bridging product intent and autonomous AI agents through rigorous [Spec-Compiled Product Engineering (SCPE)](https://github.com/m4rc0s/scpe).*

[![License](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue.svg?style=flat-square)](#-license)
[![Rust](https://img.shields.io/badge/Rust-2024%2B%20%7C%20Axum-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/Frontend-React%2019%20%7C%20Astryx-61dafb.svg?style=flat-square&logo=react)](https://react.dev/)
[![Protocol](https://img.shields.io/badge/Protocol-ACP%20(JSON--RPC%202.0)-purple.svg?style=flat-square)](#-protocol-first-agent-client-protocol-acp)
[![Methodology](https://img.shields.io/badge/Methodology-SCPE%20v0.3.0-emerald.svg?style=flat-square)](https://github.com/m4rc0s/scpe)
[![Sandbox](https://img.shields.io/badge/Sandbox-Podman%20Rootless-892CA0.svg?style=flat-square&logo=podman)](https://podman.io/)

[**Explore Features**](#-key-features) • [**Architecture**](#-architecture) • [**Repository Structure**](#-repository-structure) • [**Quick Start**](#-quick-start) • [**Documentation**](#-documentation-ecosystem)

</div>

---

## 💡 The Paradigm: Code as a Consequence

> *"The Specification is the Sovereign Contract.  
> The Document is the Interface (Document-as-UI).  
> The Agents Execute with Mathematical Precision.  
> The Code is Just the Consequence."*

Modern AI-assisted development often slips into **"vibe coding"** — unstructured, contextless code generation that creates immediate architectural debt, hallucinated dependencies, and disconnect from product requirements.

**Liquid ADE** solves this by establishing **Markdown files in Git as the Single Source of Truth (SSOT)**. It provides a visual, real-time studio for Product Managers and Tech Leads to specify, audit, and orchestrate autonomous AI agents inside isolated, rootless sandboxes.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          SINGLE SOURCE OF TRUTH                             │
│       Living Markdown Specs in Git (.md / SCPE Methodology)                 │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
         ┌─────────────────────────────┴─────────────────────────────┐
         ▼                                                           ▼
┌──────────────────────────────────────┐            ┌──────────────────────────────────────┐
│       TECHNICAL (CLI-FIRST)          │            │         PRODUCT (UI-FIRST)           │
│      Engineers & Tech Leads          │            │       PMs, POs & Designers           │
│                                      │            │                                      │
│ • Write specs in Neovim / VS Code    │            │ • Interactive WYSIWYG block editor   │
│ • Live sub-50ms sync with UI studio  │            │ • Tactile state badges & drift alert │
│ • Review diffs & TDD test passes     │            │ • Approve Readiness Gate             │
└──────────────────┬───────────────────┘            └──────────────────┬───────────────────┘
                   │                                                   │
                   └───────────────────────────┬───────────────────────┘
                                               ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                        THE CODE FRONTIER (`apps/`)                          │
│  Liquid ADE orchestrates agents; it doesn't lock you into a bloated IDE.    │
│  Downstream code lives in standard clean directories for any code editor.   │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## ✨ Key Features

### ⚡ Single-Binary, Zero-SSR Architecture
- **Ultra-Lean Rust Core:** Built with Axum and Tokio. Consumes **<50MB RSS** at idle with sub-millisecond local response times.
- **Embedded SPA:** The React 19 UI is pre-compiled and served directly from RAM using `rust-embed` — no Node.js or Bun runtime required in production.
- **Dual-Mode Delivery:** Runs seamlessly as a local/cloud web server or as a native desktop window via Tauri 2.0.

### 📝 Document-as-UI (Interactive WYSIWYG)
- **Living Specifications:** Turns raw Markdown into rich, interactive UI blocks (Linear/Notion style).
- **Surgical Line Patcher:** Direct, line-by-line updates on disk without destructive full-file rewrites.
- **Echo Suppression:** Eliminates cursor jumps and race conditions during simultaneous UI/editor changes.

### 🔄 Sub-50ms Live File Synchronization
- **Bidirectional Hot Sync:** Edit markdown in your terminal or IDE, and the Liquid interface updates instantly via WebSocket deltas.
- **Spec Drift Engine:** Real-time visual alerts when downstream implementations drift from approved specs.

### 🤖 Protocol-First: Agent Client Protocol (ACP) & LiteLLM Gateway
- **Open Standards:** Governed by JSON-RPC 2.0 over WebSocket and `stdio`.
- **Agnostic LLM Routing:** Universal connectivity to local offline models (**Ollama**, **Llama.cpp**, **vLLM**) or cloud providers (**Claude 3.5 Sonnet**, **GPT-4o**, **Gemini Pro**) via LiteLLM.

### 🛡️ Rootless Sandbox Execution (Podman)
- **Safe Autonomous Coding:** Agents compile code, install dependencies, and run TDD test suites within ephemeral, rootless Podman containers.
- **No Root Daemons:** Zero requirement for privileged Docker daemons or complex compose files.

### 🧩 Pluggable SDD (Spec-Driven Development)
- Ships with **[SCPE (Spec-Compiled Product Engineering)](https://github.com/m4rc0s/scpe)** out of the box, but allows any team to define their own specification profile via an intuitive visual configuration screen.

---

## 🏛️ Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                             LIQUID ADE INTERFACE                            │
│         React 19 • TypeScript • Astryx Design System • Zustand SPA          │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │ (JSON-RPC 2.0 / WebSocket)
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              ADE CORE (RUST)                                │
│       Axum Web Server • ACP Dispatcher • notify Async File Watcher          │
├─────────────────────────────────────────────────────────────────────────────┤
│                          LITELLM MODEL GATEWAY                              │
│       Local (Ollama / Llama.cpp) ◄────────► Cloud (Claude / OpenAI / Gemini)│
├─────────────────────────────────────────────────────────────────────────────┤
│                     PODMAN ROOTLESS SANDBOX ENGINE                          │
│          Ephemeral Container Execution • TDD Runner • Agent Scaffolding     │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 📂 Repository Structure

The workspace follows a strict **Spec-First & Monorepo** layout. Product specifications, methodology standards, and feature definitions live as the sovereign Single Source of Truth (SSOT) at the repository root and `features/`, while technical applications and source code live under `apps/`:

```text
liquid_ade/
├── 📜 product_vision.md          # Core Philosophy, Manifesto & Architectural Pillars
├── 🗺️ roadmap.md                 # Product Roadmap & Vertical Slice Milestones
├── 📖 glossary.md                # Ubiquitous Language & SCPE State Machine
├── 🧭 index.md                   # Workspace Master Index & Agent Navigation
│
├── 📂 features/                  # Canonical Product Specifications (Vertical Slices)
│   ├── 01-workspace-inspector/   # FEAT-01: Workspace & Governance Scanner
│   ├── 02-spec-status-and-drift/ # FEAT-02: State Machine & Drift Detector
│   ├── 03-live-file-sync/        # FEAT-03: Bidirectional Real-Time Sync (<50ms)
│   ├── 04-wysiwyg-document-editor/ # FEAT-04: Document-as-UI Block Editor
│   ├── 05-acp-agent-handshake/   # FEAT-05: Universal ACP & LiteLLM Gateway
│   └── 06-podman-sandbox-execution/ # FEAT-06: Rootless Podman Sandbox Runner
│
└── 📦 apps/                      # Downstream Implementations & Source Code
    └── ade/                      # Liquid ADE Application (Single-Binary Monorepo)
        ├── 📄 README.md          # 🔗 Application & Backend Documentation
        ├── 📄 app_manifest.md    # Application Topology & Architecture Manifest
        ├── 🦀 Cargo.toml         # Rust Workspace & Dependency Configuration
        ├── 🔨 build.rs           # Build script embedding the React SPA in the binary
        ├── 📂 src/               # ade_core: Axum Server, ACP Protocol, LiteLLM
        └── 📂 ui/                # ade_ui: Frontend React 19 Studio
            ├── 📄 README.md      # 🔗 Frontend Studio Documentation
            ├── 📄 UI_UX_GUIDELINES.md # Astryx UI/UX Design System Principles
            ├── 📦 package.json   # Frontend Dependencies & Scripts
            └── 📂 src/           # Components, Zustand Stores & Feature Slices
```

### 🔗 Source Code & Subsystem Documentation

Each application module contains its own dedicated technical documentation:

| Subsystem / Layer | Path | Description | Documentation |
| :--- | :--- | :--- | :--- |
| **Backend & App Host (`ade_core`)** | [`apps/ade/`](apps/ade) | Axum/Tokio web server, ACP JSON-RPC 2.0 protocol, LiteLLM gateway, Podman sandbox runner, and single-binary packaging. | [📖 Backend README](apps/ade/README.md) • [📋 App Manifest](apps/ade/app_manifest.md) |
| **Frontend Studio (`ade_ui`)** | [`apps/ade/ui/`](apps/ade/ui) | React 19 SPA, Astryx Design System, block-based WYSIWYG specification editor, state machine badges, and live sync deltas. | [📖 Frontend README](apps/ade/ui/README.md) • [🎨 UI/UX Guidelines](apps/ade/ui/UI_UX_GUIDELINES.md) |
| **Feature Specifications** | [`features/`](features) | Detailed PRDs, epic definitions, tasks, and state tracking for all core features. | [🧭 Master Index](index.md) • [🗺️ Product Roadmap](roadmap.md) |

---

## 🚀 Quick Start

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.80+) & `cargo`
- [Bun](https://bun.sh/) (for frontend development)
- [Podman](https://podman.io/) (optional, for agent sandboxed execution)

### 1. Clone the Repository
```bash
git clone https://github.com/your-org/liquid-ade.git
cd liquid-ade
```

### 2. Run in Development Mode
Start the frontend and backend with hot reload:

```bash
# Terminal 1: Run the React UI (ade_ui)
cd apps/ade/ui
bun install
bun run dev

# Terminal 2: Run the Rust Core (ade_core)
cd apps/ade
cargo run
```
Open [http://localhost:3000](http://localhost:3000) in your browser.

### 3. Build Self-Contained Release Binary
Build a standalone binary with the frontend fully embedded:

```bash
cd apps/ade
cargo build --release
```
The resulting executable in `./target/release/liquid-ade` is a single binary with zero external runtime dependencies.

---

## 📚 Documentation Ecosystem

- **[product_vision.md](product_vision.md)** — Core Philosophy, Manifesto, and Architectural Pillars.
- **[roadmap.md](roadmap.md)** — Product Roadmap, Timeline, and Vertical Slices Matrix.
- **[glossary.md](glossary.md)** — Ubiquitous Language & [SCPE](https://github.com/m4rc0s/scpe) State Machine (`Draft → Ready → WIP → Done`).
- **[index.md](index.md)** — Workspace Master Index & Agent Navigation Entrypoint.
- **[Application Backend README](apps/ade/README.md)** — Backend architecture, ACP protocol, and single-binary packaging.
- **[Application Frontend README](apps/ade/ui/README.md)** — Frontend architecture, Astryx design system, and UI components.
- **[UI/UX Guidelines](apps/ade/ui/UI_UX_GUIDELINES.md)** — Astryx Design System Principles and Accessibility Heuristics.

---

## 🤝 Contributing

We welcome contributions from developers, designers, and product practitioners!
1. Fork the repository
2. Create your feature branch (`git checkout -b feat/my-new-feature`)
3. Commit your changes following [Conventional Commits](https://www.conventionalcommits.org/) (`git commit -m 'feat(sync): add binary diff watcher'`)
4. Push to the branch (`git push origin feat/my-new-feature`)
5. Open a Pull Request

---

## 📄 License

Distributed under the **MIT** or **Apache-2.0** License.
