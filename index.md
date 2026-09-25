# 🌊 Liquid ADE: Workspace Master Guide

**Methodology:** SCPE v0.3.0 (Spec-Compiled Product Engineering)  
**Workspace Status:** Active / Canonical Single Source of Truth (SSOT)  
**Last Updated:** 2026-08-31  

---

## 1. Workspace Overview

The **Liquid** workspace is strictly governed by the **SCPE v0.3.0** methodology. The repository centralizes all living product specifications at the root and within feature folders, while the technical implementation resides under `apps/ade/`:

```text
/home/mraraujo/product_design/liquid_ade/
├── 📜 product_vision.md          # Product Vision, Manifesto & Architectural Pillars
├── 🗺️ roadmap.md                 # Planning for the 6 Vertical Slices
├── 📖 glossary.md                # Ubiquitous Language and State Machine
├── 🧭 index.md                   # This Master Guide
│
├── 📦 apps/
│   └── ade/                      # Monorepo Rust (Axum/Tokio) + React (Astryx SPA)
│       ├── app_manifest.md       # Application technical manifest
│       ├── Cargo.toml            # Backend orchestrator
│       ├── build.rs              # Build script embedding the UI into the binary
│       └── ui/
│           ├── UI_UX_GUIDELINES.md # Constitution with 7 UI Pillars and 8 Heuristics
│           └── package.json      # React 19 + Astryx Design System
│
├── 📂 features/                  # The 6 Business Vertical Slices
│   ├── 01-workspace-inspector/   # Project viewer and Markdown reader
│   ├── 02-spec-status-and-drift/ # Tactile badges and Spec Drift detection
│   ├── 03-live-file-sync/        # Real-time synchronization (<50ms) via Watcher
│   ├── 04-wysiwyg-document-editor/ # WYSIWYG block editor with surgical line patcher
│   ├── 05-acp-agent-handshake/   # ACP Protocol and LiteLLM routing
│   └── 06-podman-sandbox-execution/ # Agent execution in Podman container
│
└── 🏛️ archive/
    └── v0.5-prototype-legacy/    # Legacy prototype preserved as history
```

---

## 2. Vertical Slices Matrix (`features/`)

Each folder inside `features/` is a **self-contained Bounded Context** structured with:
- `index.md` (Feature PRD and end-to-end user journey).
- `feat_roadmap.md` (Tracking internal epics).
- `quick_status.md` (Formal state: `Draft`, `Ready`, `WIP`, `Done`, `Blocked`, `Stale`).
- Subfolders `epic_01_*/` and `epic_02_*/` containing their respective `index.md`, `plan.md`, `tasks.md`, and `quick_status.md`.

## Features

- [Inception Studio & Idea Validation](features/02-inception-studio/index.md)
