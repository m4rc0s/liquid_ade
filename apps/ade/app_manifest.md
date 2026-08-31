# App Manifest: liquid-ade

- **app_name:** liquid-ade
- **app_type:** single-binary-agentic-ide
- **tech_stack:** Rust, Axum, Tokio, React 19, Vite, TypeScript, Astryx Design System (`@astryxdesign/core`), Zustand, LiteLLM, Podman
- **design_patterns:** Single Binary Embedding (`rust-embed`), Agent Client Protocol (ACP) JSON-RPC 2.0 Server, Static SPA Architecture (Zero SSR), Rootless Container Sandboxing (Podman)
- **app_description:** Autonomous engine and Agentic Development Environment (ADE) for Liquid. Written in Rust for performance and security. The React frontend (Astryx SPA) is served by Axum for Web/SaaS and embedded into the standalone binary or native desktop window (Tauri).
- **entrypoint:** src/main.rs (Rust Server) / ui/src/main.tsx (React Client)
- **dependencies_scope:** 
  - Communicates with the local filesystem (workspace/features) in a strictly isolated manner.
  - Implements an ACP (Agent Client Protocol) server and utilizes LiteLLM via HTTP calls to access open/local models (Ollama, vLLM) or cloud providers (Claude, OpenAI, Gemini).
  - Executes isolated downstream code tasks inside ephemeral containers via Podman (without docker-compose).
  - Zero dependency on Node.js/Bun in production environments.

---
> Note: Business guidelines, roadmap, epics, and domain rules are located at the root of the workspace in `product_vision.md` and the `features/` directory. This manifest defines the technical boundaries of the application.
