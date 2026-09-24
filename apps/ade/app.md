# App: ade

- **name:** ade
- **type:** single-binary-agentic-ide
- **description:** Liquid ADE (Agentic Development Environment). Autonomous engine and developer environment for Spec-Compiled Product Engineering.
- **stack:** Rust, Axum, Tokio, React 19, Vite, TypeScript, Astryx Design System, Zustand, LiteLLM, Podman
- **standards:** as agreed in technical_deal.md
- **entrypoint:** src/main.rs (Rust) / ui/src/main.tsx (React)
- **depends_on:** local filesystem, Google Gemini API / LiteLLM gateway, Podman runtime
- **run:** cargo run
- **test:** cargo test

## Boundaries

- Must not store state in proprietary or hidden external databases; the local filesystem is the SSOT.
- Must not execute agent tasks directly on the host system with root privileges; all execution must go through Podman sandboxes.
- Must not introduce Node.js or SSR dependencies in the compiled production distribution.
- Must not allow path traversal beyond the registered workspace directory root.
