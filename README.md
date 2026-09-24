# 🌊 Liquid ADE

> **The Spec-First, Agentic Development Environment (ADE).**  
> Transform living Markdown specifications into deterministic software with mathematical precision.

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-Phase%202%20Inception-yellow.svg)](quick_status.md)
[![Method](https://img.shields.io/badge/method-SCPE%20v0.3.0-emerald.svg)](https://github.com/m4rc0s/scpe)
[![Rust](https://img.shields.io/badge/backend-Rust%202021%20%7C%20Axum-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/ui-React%2019%20%2B%20Astryx-61dafb.svg?logo=react)](https://react.dev/)
[![Database](https://img.shields.io/badge/db-SQLite%20Embedded-003b57.svg?logo=sqlite)](https://www.sqlite.org/)

---

## 🎯 The Problem

Modern AI-assisted software generation has plunged into **"vibe coding"** — unguided, unstructured prompts producing code detached from real product intent. Teams face:

- **Requirements Disconnect:** Requirements scatter across Jira, Notion, Slack, and PR descriptions, while source code drifts aimlessly.
- **Architectural Vacuum & Context Loss:** Autonomous agents operate without rigid domain boundaries (*Bounded Contexts*), hallucinating libraries, endpoints, and database models.
- **Absence of Acceptance Verification:** Code lands in repositories without passing formal, measurable acceptance criteria.
- **Bloated IDE Duplication:** Most AI developer tools attempt to clone full code editors inside the browser, creating sluggish web UIs that try to replace your trusted native environment.

Today, architecture lives in memory, and software correctness is left to chance.

## 💡 The Solution

**Liquid ADE** embraces **Spec-Driven Development (SDD)** under the **SCPE v0.3.0** (*Spec-Compiled Product Engineering*) methodology:

> *"The Specification is the Sovereign Contract. The Document is the Interface (Document-as-UI). The Agents Execute with Mathematical Precision. The Code is Just the Consequence."*

- ✅ **Living Specs as Sovereign SSOT** — Structured Markdown files in Git (`.md`) are the inviolable Single Source of Truth.
- ✅ **Dual-Ledger Session Architecture** — High-velocity transcript streaming alongside a durable SQLite operational state ledger (`~/.liquid/liquid.db`).
- ✅ **Living Spec Canvas & 1-Click Task Runner** — Specification tasks (`tasks.md`) project as interactive action cards with 1-click execution triggers (`▶ Start task`).
- ✅ **Zero-SSR Single Binary** — Ultra-fast Axum core embedding the static React 19 SPA (`rust-embed`) consuming **<50MB RAM** with sub-100ms startup.
- ✅ **Clean Code Frontier (`apps/`)** — Zero browser editor lock-in. Agent-generated code resides cleanly on disk under `apps/` for your external editor (Neovim, VS Code, Cursor).

## 📊 What You Get

### For Technical Leaders & Senior Engineers
- **Mathematical Precision:** Epics and tasks anchored to formal business rules (`R#`) and BDD acceptance scenarios (`S#`).
- **Acceptance Verification Gate:** Automated testing enforces scenario validation before any task can be marked complete (`- [x]`).
- **Pristine Linear Git History:** Pure bifurcation-free `main` branch with curated squash merges and English Conventional Commits.
- **Strict Traversal Guards:** Secure filesystem reader protecting the host operating system from path escape attacks (`TRAVERSAL_DETECTED`).

### For Product Managers, Designers & Stakeholders
- **Visual Project Management:** Jira-style Liquid Board projecting real-time lifecycle states (`Draft` → `Ready` → `WIP` → `Done`).
- **Conversational Inception Studio:** Dialogue with multi-LLM co-pilots (powered by Google Gemini) to synthesize product vision, guidelines, and architecture.
- **Readiness Gate Governance:** Formal human sign-off required before an epic can transition from specification into execution.
- **Zero Terminal Overhead:** Complete product inception, scoping, and governance without requiring command-line or code editor access.

## 🏗️ Roadmap

| Phase | Milestone | Focus | Status |
| :---: | :--- | :--- | :---: |
| **Phase 1** | **v0.1.0 The Inspector** | Axum runtime shell, static React 19 SPA fallback, safe FS tree scanner & traversal guard | ✅ Done |
| **Phase 2** | **v0.2.0 Inception Studio** | Project scaffolding, Gemini co-pilot streaming, living spec canvas, 1-click task runner | 🚀 In Progress |
| **Phase 3** | **v0.3.0 The Synchronizer** | Async file watching (`notify`), bidirectional WebSocket live sync (<50ms), echo suppression | 📋 Backlog |
| **Phase 4** | **v0.4.0 The Auditor** | Workspace status matrix, tactile Astryx state badges, and spec drift inspector | 📋 Backlog |
| **Phase 5** | **v0.5.0 The Studio** | Rich WYSIWYG document editor with semantic AST block model and surgical line patcher | 📋 Backlog |
| **Phase 6** | **v0.6.0 The Protocol** | Agent Client Protocol (ACP) via JSON-RPC 2.0 and LiteLLM universal multi-model gateway | 📋 Backlog |
| **Phase 7** | **v0.7.0 The Engine** | Ephemeral rootless Podman sandbox factory for isolated downstream execution | 📋 Backlog |

## 🛠️ Tech Stack

### Backend Engine
- **Language:** Rust 2021+
- **HTTP & Async Runtime:** Axum 0.7, Tokio 1.0, Tower
- **Database:** Embedded SQLite via bundled `rusqlite` 0.32 (`~/.liquid/liquid.db`)
- **Asset Embedding:** `rust-embed` (pre-compiled React SPA baked into the binary)
- **Serialization:** Serde, Serde JSON

### Frontend Studio
- **Framework:** React 19
- **Language:** TypeScript 5.8+
- **Build Tool:** Vite 8
- **Design System:** Astryx Design System (`@astryxdesign/core`)
- **State Management:** Zustand
- **Tooling:** Bun, Oxlint, Prettier

### Methodology & Automation
- **Methodology:** SCPE v0.3.0 (*Spec-Compiled Product Engineering*)
- **Task Runner:** Just (`Justfile`)
- **Git Strategy:** Option B (PR-based review & linear Squash Merge on GitHub)

## 🚀 Quick Start

### Prerequisites
- [Rust & Cargo](https://www.rust-lang.org/tools/install) (v1.80+)
- [Bun](https://bun.sh/) (for frontend asset building)
- [Just](https://github.com/casey/just) (command runner)
- Git & GitHub CLI (`gh`)

### Setup

```bash
# Clone the repository
git clone https://github.com/m4rc0s/liquid_ade.git
cd liquid_ade

# Validate specifications, linters, and type checking
just check

# Run the test suite
just test

# Start the local development server (Axum + Embedded SPA)
just dev
```
Open [http://localhost:3000](http://localhost:3000) in your browser to access the Liquid ADE interface.

### Available Commands

```bash
# Validation & Code Quality
just check               # Run strict SCPE validation, formatting checks, clippy, oxlint, and tsc
just lint                # Run clippy (Rust) and oxlint (TypeScript)
just fmt                 # Auto-format Rust (cargo fmt) and frontend (prettier)

# Testing & Execution
just test                # Run all backend unit and BDD integration tests
just dev                 # Launch development server
just build               # Compile standalone production release binary

# Team Workflow (Option B)
just branch-start <name> # Create feature branch from latest main
just branch-push <name>  # Push feature branch to remote origin
just sync                # Synchronize local main with origin after PR merge
```

## 🏗️ Project Structure

```text
liquid_ade/
├── 📜 product_vision.md          # Sovereign Manifesto, Architecture & Product Vision
├── 🗺️ roadmap.md                 # Macro Roadmap & Incremental Evolution Horizons
├── 📖 glossary.md                # Ubiquitous Domain Language (SCPE State Machine)
├── 🏛️ architecture.md            # System Architecture & Technical Boundaries
├── 🤝 technical_deal.md          # Technical Deal, Approved Stack & Linters
├── 📋 team_playbook.md           # Team Playbook: Option B PR Workflow & Commits
├── 🚦 quick_status.md            # Product-Wide Status Dashboard
├── 🧭 index.md                   # Master Entrypoint for People & Autonomous Agents
│
├── 📂 features/                  # Vertical Slices (Living Specifications)
│   ├── 01-workspace-inspector/   # Phase 1: Shell, Traversal Guard & Tree API (Done)
│   └── 02-inception-studio/      # Phase 2: Scaffolding, Gemini Streaming & Board (Active)
│
├── 📦 apps/                      # Downstream Implementations (Code Frontier)
│   └── ade/                      # Single-Binary Liquid ADE Application
│       ├── 🦀 Cargo.toml         # Rust Workspace & Dependencies
│       ├── 📂 src/               # Axum server, PathGuard, Scaffold engine & SQLite
│       ├── 📂 tests/             # BDD Acceptance Tests (<epic>#S#)
│       └── 📂 ui/                # React 19 Frontend Studio (Astryx Design System)
│
└── ⚙️ Justfile                   # Deterministic Automation SSOT
```

## 📖 Documentation

**Core Steering Documents:**
1. **[product_vision.md](product_vision.md)** — Macro vision, concept-to-production continuum, and architectural pillars
2. **[roadmap.md](roadmap.md)** — Macro roadmap and horizontal phase transitions
3. **[glossary.md](glossary.md)** — Ubiquitous domain language and terminology
4. **[index.md](index.md)** — Master workspace index and navigation guide

**Engineering & Architecture:**
- **[architecture.md](architecture.md)** — Single-binary architecture, embedded SQLite, and system boundaries
- **[technical_deal.md](technical_deal.md)** — Approved stack, linters, and quality standards
- **[team_playbook.md](team_playbook.md)** — Option B PR-based review workflow, squash merge rules, and commit conventions
- **[CHANGELOG.md](CHANGELOG.md)** — Semantic Versioning and milestone changelog

**Status & Traceability:**
- **[quick_status.md](quick_status.md)** — Active epic states, blockers, and confidence metrics
- **[features/](features/)** — Feature and epic specifications, Given/When/Then scenarios, and task lists

## 🤝 Contributing

This project is governed by **SCPE (Spec-Compiled Product Engineering)**: Markdown specs in Git are the Single Source of Truth.

**Team Workflow (Option B):**
1. Create a branch from `main`: `just branch-start <epic-slug>`
2. Implement tasks in `tasks.md`, following test-driven verification (`<epic>#S<scenario>`)
3. Verify quality: `just check && just test`
4. Push branch and open a Pull Request: `just branch-push <epic-slug>`
5. Tech Lead reviews and performs **Squash and Merge** on GitHub
6. Synchronize locally: `just sync`

Learn more: [SCPE Methodology on GitHub](https://github.com/m4rc0s/scpe)

## 🧪 Testing

```bash
# Run all automated tests
just test
# or: cargo test

# Run tests for a specific epic suite
cargo test --test epic_01_runtime_shell_tests
cargo test --test epic_02_workspace_fs_scanner_tests
cargo test --test epic_01_project_scaffold_generator_tests
```

All integration tests are strictly organized by epic and acceptance scenario: `<epic_name>#S<scenario>` (e.g. `epic_01_project_scaffold_generator#S1`).

## 📋 Status

**Current State:** Phase 2 Inception Studio (Active Implementation)  
**Completed Epics:**
- `01-workspace-inspector/epic_01_runtime_shell` — `Done` ✅
- `01-workspace-inspector/epic_02_workspace_fs_scanner` — `Done` ✅
- `02-inception-studio/epic_01_project_scaffold_generator` — `Done` ✅

**Active / Next Epic:**
- `02-inception-studio/epic_02_conversational_copilot_panel` — `Draft` (Gate Checked) 📋

**Blockers:** None

See [quick_status.md](quick_status.md) and [features/02-inception-studio/quick_status.md](features/02-inception-studio/quick_status.md) for granular details.

## 📄 License

Dual-licensed under the **MIT** or **Apache 2.0** License.

---

**Built with ❤️ by the Liquid ADE team.**
