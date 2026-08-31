# ✨ ade_ui — Liquid ADE Frontend

<div align="center">

### *The High-Density, Document-as-UI Studio for Autonomous Engineering*
**React 19 • TypeScript • Vite • Astryx Design System • Zustand**

</div>

---

## 📖 Overview

**`ade_ui`** is the high-performance visual studio for **Liquid ADE**. Inspired by the tactile interaction patterns and refined craft of tools like **Linear** and **Notion**, it transforms living Markdown specifications into a structured WYSIWYG block editor with tactile feedback and sub-50ms bidirectional synchronization.

The frontend is designed to be compiled and embedded directly into the standalone Rust backend (`ade_core`) binary via `rust-embed`, requiring zero Node.js/Bun runtime in production environments.

---

## 🎨 Astryx Design System & UI/UX Philosophy

The interface is governed by the principles defined in [UI/UX Guidelines](UI_UX_GUIDELINES.md):

### 🏛️ The 7 UI Pillars
1. **Linear-like Typography:** Highly scannable hierarchy with distinct weights (`font-semibold`), breathable body text, and Monospace styling for metadata and [SCPE](https://github.com/m4rc0s/scpe) tags.
2. **Sophisticated Dark Mode & Palette:** Deep gray layers with ultra-subtle borders (`1px solid rgba(255, 255, 255, 0.06)`). Semantic status badges for [SCPE](https://github.com/m4rc0s/scpe) states (`Draft`, `Ready`, `WIP`, `Done`, `Blocked`).
3. **Astryx Component Craft:** Tactile components with smooth corner radii (6px to 8px) and snappy transitions (100ms).
4. **Linear Density & Grid:** Modular 4px/8px layout grid with clear breathing room focused on the specification document.
5. **Visual Hierarchy:** The document canvas is the central protagonist; menus and sidebars remain subtle and collapsible.
6. **Iconography:** Minimalist icons (Lucide) with a crisp 1.5px stroke width.
7. **Tactile Feedback:** Real-time micro-indicators for disk synchronization and ACP connection status.

### 🧭 UX Accelerators & Heuristics
* **`Cmd+K` / `Ctrl+K` (Command Palette):** Instant navigation between features, epic search, and quick actions.
* **Slash Menu (`/`):** Quick insertion of acceptance criteria, task blocks, and spec sections into the editor.
* **Full Undo / Redo (`Cmd+Z` / `Cmd+Shift+Z`):** Comprehensive local history management during editing sessions.

---

## 🛠️ Tech Stack

| Layer | Technology | Purpose |
| :--- | :--- | :--- |
| **Framework** | [React 19](https://react.dev/) | High-performance reactive and concurrent rendering |
| **Language** | [TypeScript 5+](https://www.typescriptlang.org/) | Strict end-to-end type safety |
| **Bundler / HMR** | [Vite 6+](https://vite.dev/) | Ultra-fast builds with Hot Module Replacement |
| **Design System** | [Astryx Design System](https://github.com/m4rc0s/astryx) / Tailwind CSS | High-density component library and utility styling |
| **Global State** | [Zustand](https://zustand-demo.pmnd.rs/) | Lightweight, decoupled global state management |
| **Protocol** | ACP (Agent Client Protocol) / JSON-RPC 2.0 | Real-time WebSocket communication with `ade_core` |
| **Linter** | [Oxlint](https://oxc.rs/) | Blazing-fast Rust-based static linter |

---

## 📂 Directory Structure

```text
ui/
├── public/                # Favicons, SVGs and static public assets
├── src/
│   ├── assets/            # Application imagery and illustrations
│   ├── components/        # Reusable UI components (Astryx & customized shadcn/ui)
│   ├── features/          # Vertical slices (Package-by-Feature)
│   │   ├── workspace/     # Filesystem reader and spec indexer
│   │   ├── editor/        # WYSIWYG Document-as-UI editor and AST blocks
│   │   ├── status-matrix/ # State machine auditor and drift detector
│   │   └── protocol/      # ACP control panel and agent orchestrator
│   ├── services/          # WebSocket / ACP client and REST API helpers
│   ├── stores/            # Global Zustand stores (workspace, editor, acp)
│   ├── App.tsx            # Main application shell
│   ├── main.tsx           # React 19 entrypoint
│   └── index.css          # Design tokens and theme CSS variables
├── package.json           # Dependencies and NPM scripts
├── tsconfig.json          # TypeScript compiler configuration
└── vite.config.ts         # Vite build configuration
```

---

## 🚀 Available Scripts

Use [Bun](https://bun.sh/) to manage dependencies and run tasks:

### Development
```bash
# Start local development server with Hot Reload
bun run dev
```
Opens the interface at [http://localhost:5173](http://localhost:5173).

### Static Analysis & Linting
```bash
# Run static analysis with Oxlint
bun run lint
```

### Testing
```bash
# Run unit and integration test suites
bun test
```

### Production Build
```bash
# Typecheck (tsc) and compile static production bundle into `dist/`
bun run build
```
*Note: The Rust build script (`build.rs` in `apps/ade`) executes this command automatically when compiling the release binary.*

### Bundle Preview
```bash
# Preview the generated static build from `dist/`
bun run preview
```

---

## 📚 Related Documentation

* [🎨 UI/UX Guidelines (Design Principles)](UI_UX_GUIDELINES.md)
* [📖 Backend / Host Application README (`apps/ade`)](../README.md)
* [📋 Technical Application Manifest](../app_manifest.md)
* [🌐 Root Product & Workspace README](../../../README.md)
* [📐 SCPE Official Methodology Repository](https://github.com/m4rc0s/scpe)
