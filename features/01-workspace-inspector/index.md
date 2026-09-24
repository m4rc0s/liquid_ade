# Feature: Workspace Inspector & Inception Studio

- **goal:** Enable local-first workspace inspection and interactive product inception via conversational co-pilot for idea validation under SCPE v0.3.0
- **users:** Senior Architects, Product Engineers, Tech Leads, and Autonomous AI Agents
- **bounded_context:** Workspace Navigation & Inception
- **business_value:** Drastically cuts product ideation cycle time, ensuring projects start with canonical SCPE specifications before any code is generated

## Scope
- Scanning and verifying local workspace directory structures against SCPE standards.
- Scaffolding new SCPE product workspaces with canonical root files and directories.
- Kimi-inspired dual-pane conversational co-pilot for guided drafting and refinement of `product_vision.md`, `UI_UX_GUIDELINES.md`, and `architecture.md` (including ADRs).
- Visual and conversational interactive roadmap planning.
- Validated handoff from Inception to downstream Feature Studio.

## Out of Scope
- Direct agent code compilation and test execution (handled by FEAT-06 Podman Sandboxing).
- Two-way surgical AST block editing on arbitrary code files (handled by FEAT-04 WYSIWYG Editor).

## Epics

- [epic_01_runtime_shell](epics/epic_01_runtime_shell/index.md) — Draft
- [epic_02_workspace_fs_scanner](epics/epic_02_workspace_fs_scanner/index.md) — Draft
- [epic_03_project_scaffold_generator](epics/epic_03_project_scaffold_generator/index.md) — Draft
- [epic_04_conversational_copilot_panel](epics/epic_04_conversational_copilot_panel/index.md) — Draft
- [epic_05_product_vision_refiner](epics/epic_05_product_vision_refiner/index.md) — Draft
- [epic_06_design_guidelines_builder](epics/epic_06_design_guidelines_builder/index.md) — Draft
- [epic_07_architecture_and_adr_engine](epics/epic_07_architecture_and_adr_engine/index.md) — Draft
- [epic_08_interactive_roadmap_adjuster](epics/epic_08_interactive_roadmap_adjuster/index.md) — Draft
- [epic_09_feature_planning_handoff](epics/epic_09_feature_planning_handoff/index.md) — Draft
