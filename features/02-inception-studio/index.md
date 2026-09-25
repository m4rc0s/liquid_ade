# Feature: Inception Studio & Idea Validation

- **goal:** Enable conversational product inception, architectural formulation, visual project management via Liquid Board, and one-click task execution using multi-LLM routing (Google Gemini).
- **users:** Product Managers, Senior Architects, Tech Leads, and Autonomous Agents
- **bounded_context:** Inception & Visual Product Engineering
- **business_value:** Allows users to define products conversationally, scaffold canonical SCPE documents, track progress on a Jira-style board, and execute tasks with 1 click.

## Scope
- Workspace scaffolding modal and project creation (`epic_01_project_scaffold_generator`).
- Conversational co-pilot panel with Gemini REST streaming (`epic_02_conversational_copilot_panel`).
- Technical deal stack compliance & Astryx foundation (`epic_03_technical_deal_compliance`).
- Living Spec Canvas shell with Mode Rail, Spec Navigator, and Inspector/Co-Pilot tabs (`epic_04_living_spec_canvas_shell`).
- Product vision refinement and live document patching (`epic_05_product_vision_refiner`).
- Design guidelines builder for UI/UX standards (`epic_06_design_guidelines_builder`).
- Architecture guidelines and ADR generator (`epic_07_architecture_and_adr_engine`).
- Interactive roadmap visualizer and conversational adjuster (`epic_08_interactive_roadmap_adjuster`).
- Inception-to-Feature handoff and feature scaffolding (`epic_09_feature_planning_handoff`).
- Jira-style Liquid Board and One-Click Interactive Task Runner (`epic_10_liquid_board_and_task_runner`).

## Out of Scope
- Code editing in browser (Liquid delegates code editing to developer's external IDE).
- Live WebSocket file watching with echo suppression (Phase 3: Live File Sync).
- Sandboxed code execution in Podman (Phase 7: Podman Sandbox Runner).

## Epics

- [epic_01_project_scaffold_generator](epics/epic_01_project_scaffold_generator/index.md) — Done
- [epic_02_conversational_copilot_panel](epics/epic_02_conversational_copilot_panel/index.md) — Done
- [epic_03_technical_deal_compliance](epics/epic_03_technical_deal_compliance/index.md) — Done
- [epic_04_living_spec_canvas_shell](epics/epic_04_living_spec_canvas_shell/index.md) — Done
