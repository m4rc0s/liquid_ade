# App Manifest: liquid-ade

- **app_name:** liquid-ade
- **app_type:** single-binary-agentic-ide
- **tech_stack:** Rust, Axum, Tokio, React 19, Vite, TypeScript, Astryx Design System (`@astryxdesign/core`), Zustand, LiteLLM, Podman
- **design_patterns:** Single Binary Embedding (`rust-embed`), Agent Client Protocol (ACP) JSON-RPC 2.0 Server, Static SPA Architecture (Zero SSR), Rootless Container Sandboxing (Podman)
- **app_description:** Motor autônomo e ADE do Liquid. Escrito em Rust para performance e segurança. O frontend em React (Astryx SPA) é servido pelo Axum na Web/SaaS e embutido no binário ou janela nativa (Tauri Desktop).
- **entrypoint:** src/main.rs (Rust Server) / ui/src/main.tsx (React Client)
- **dependencies_scope:** 
  - Comunica-se com o sistema de arquivos local (workspace/features) de forma estritamente isolada.
  - Implementa um servidor ACP (Agent Client Protocol) e utiliza LiteLLM via chamadas HTTP para acessar modelos abertos/locais (Ollama, vLLM) ou nuvem (Claude, OpenAI, Gemini).
  - Execução de tarefas de código downstream isoladas em contêineres efémeros via Podman (sem docker-compose).
  - Zero dependência de Node.js/Bun em ambiente de produção.

---
> Nota: As diretrizes de negócio, roadmap, épicos e regras de domínio estão localizados na raiz do workspace, em `product_vision.md` e na estrutura da pasta `features/`. Este arquivo define o perímetro técnico da aplicação.
