# 🎨 UI/UX Guidelines & Core Principles (Linear & Notion Style)

This document establishes the **Core UI Principles and UX Heuristics** for the interfaces of **Liquid ADE**, directly inspired by the design and interaction patterns of **Linear** and **Notion**: minimalist aesthetics, refined typography, document-centric layouts, high information density, keyboard accelerators, and tactile micro-interactions.

> **Base Component Library:** [Astryx Design System](https://github.com/m4rc0s/astryx) (`@astryxdesign/core` - React 19+)

---

## 🏛️ PART 1: The 7 UI Pillars (Linear / Notion Style)

1. **Typography (Linear-like):**
   - Clean, highly scannable hierarchy: prominent headlines with distinct weight (`font-semibold`), breathable body text with comfortable line-height (`leading-relaxed`), and strict Monospace font for metadata, identifiers, and tags.
2. **Colors & Surfaces (Sophisticated Dark Mode):**
   - Deep, layered dark grays with ultra-subtle borders (`1px solid rgba(255,255,255,0.06)`).
   - Refined semantic status colors: `Done` (Emerald green), `Ready` (Electric blue), `WIP` (Amber/Orange), `Blocked` (Crimson red), `Draft` (Neutral gray).
3. **Component Craft (Astryx):**
   - Buttons, context menus, modals, and badges styled with smooth corner radii (6px to 8px) and instantaneous transitions (100ms).
4. **Layout & Density (Linear Style):**
   - 4px/8px modular grid. Subtle dividers without heavy borders or artificial drop-shadows. Visual breathing room centered around the specification document.
5. **Visual Hierarchy:**
   - The active document on the canvas is the primary focal point. Menus and sidebars remain unobtrusive and collapsible.
6. **Iconography:**
   - Minimalist, crisp icons (Lucide) with a uniform 1.5px stroke width and consistent sizing.
7. **Feedback & Micro-Interactions:**
   - Tactile hover states, subtle focus rings, and instantaneous synchronization micro-indicators ("*Saved*").

---

## 🧭 PART 2: The 8 UX Heuristics (Notion/Linear Interaction)

1. **Visibility of System Status:** Real-time ACP connection indicators and disk-save status indicators.
2. **Match Between System and Real World:** Clear, familiar product terminology ("*Epics*", "*Tasks*", "*Acceptance Criteria*").
3. **User Control & Freedom:** Complete **Undo/Redo (`Cmd+Z` / `Cmd+Shift+Z`)** support in the specification editor.
4. **Consistency:** SCPE navigation sidebar on the left, central document canvas, and seamless toggle to the Liquid Board (Kanban).
5. **Error Prevention:** Gentle input validations and prevention of illegal state transitions.
6. **Cognitive Load Reduction:** Zen-like interface optimized for reading and writing without visual clutter.
7. **Minimalist Aesthetic:** Elimination of decorative noise; absolute focus on specification content.
8. **Accelerators (Power Users - Linear/Notion Standards):**
   - **`Cmd+K` (Command Palette):** Rapid feature navigation, epic search, and action commands.
   - **Slash Menu (`/`):** Fast insertion of text blocks, criteria checklists, and spec sections within the editor.
