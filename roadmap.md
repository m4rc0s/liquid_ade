# 🗺️ Liquid: Macro Roadmap & Release Strategy

**Metodologia:** SCPE v0.3.0 (Spec-Compiled Product Engineering)  
**Status Global:** Ativo / Fonte Canônica de Evolução Temporal  
**Última Atualização:** 2026-08-31  
**Mantido por:** Product Architecture & Engineering Council  

---

## 1. Visão Temporal & Estratégia de Releases em 6 Fases Granulares

O desenvolvimento do **Liquid** é fatiado em **6 Fases Progressivas e Incrementais**. Cada fase entrega um marco de produto funcional, testável e com escopo pequeno e hiper-focado:

```
2026 Q3                                  2026 Q4                                  2027 Q1
  │                                        │                                        │
  ├─ Fase 1: v0.1.0 (The Reader) ──────────┼─ Fase 3: v0.3.0 (The Synchronizer) ────┼─ Fase 5: v0.5.0 (The Protocol) ────▶
  │  FEAT-01: Workspace Inspector          │  FEAT-03: Live File Sync               │  FEAT-05: ACP & LiteLLM Gateway
  │                                        │                                        │
  ├─ Fase 2: v0.2.0 (The Auditor) ─────────┼─ Fase 4: v0.4.0 (The Studio) ──────────┼─ Fase 6: v0.6.0 (The Engine) ──────▶
  │  FEAT-02: Spec Status & Drift          │  FEAT-04: WYSIWYG Document Editor      │  FEAT-06: Podman Sandbox Runner
```

---

## 2. Detalhamento das 6 Fases de Evolução

### 🔹 Fase 1: v0.1.0 — The Reader (O Visualizador de Governança)
* **Objetivo:** Ter uma ferramenta funcional no Dia 1 que escaneia e exibe qualquer workspace SCPE de forma elegante.
* **Feature:** **`FEAT-01: Workspace Inspector`**
* **Jornada do Usuário:** O usuário abre o app no navegador (`localhost:3000`), vê a árvore de governança na barra lateral e lê os documentos Markdown renderizados no canvas com tipografia Astryx.
* **Entregáveis:**
  - Servidor Axum em Rust servindo a SPA estática embutida.
  - Scanner de arquivos lendo `product_vision.md`, `roadmap.md` e pastas `features/`.

---

### 🔹 Fase 2: v0.2.0 — The Auditor (O Painel de Estados & Integridade)
* **Objetivo:** Adicionar rastreabilidade visual e auditoria do ciclo de vida de produto.
* **Feature:** **`FEAT-02: Spec Status & Drift`**
* **Jornada do Usuário:** O usuário visualiza badges táteis (`Draft`, `Ready`, `WIP`, `Done`) ao lado de cada épico e recebe alertas visuais automáticos de *Spec Drift* se o `plan.md` sofrer modificações pós-entrega.
* **Entregáveis:**
  - Leitor de `quick_status.md` em toda a árvore do projeto.
  - Componente de Badges semânticos com Astryx (Verde, Âmbar, Vermelho, Neutro).
  - Algoritmo de validação de drift e banner de inconsistência.

---

### 🔹 Fase 3: v0.3.0 — The Synchronizer (A Sincronização em Tempo Real)
* **Objetivo:** Conectar a interface do Liquid ao fluxo de trabalho tradicional no VS Code/Neovim.
* **Feature:** **`FEAT-03: Live File Sync`**
* **Jornada do Usuário:** O usuário edita o arquivo Markdown no seu editor favorito e a tela do Liquid ADE atualiza instantaneamente (<50ms) sem recarregar e sem perder a posição de leitura.
* **Entregáveis:**
  - File Watcher assíncrono em Rust (`notify`) monitorando arquivos `.md`.
  - Canal WebSocket (`/ws/workspace`) transmitindo deltas para a store Zustand.
  - Micro-indicador visual de status de sincronização.

---

### 🔹 Fase 4: v0.4.0 — The Studio (O Editor WYSIWYG Document-as-UI)
* **Objetivo:** Transformar o Markdown em uma interface visual rica e editável com gravação cirúrgica no disco.
* **Feature:** **`FEAT-04: WYSIWYG Document Editor`**
* **Jornada do Usuário:** O usuário clica em checklists de critérios de aceite, chips de metadados ou tabelas DDD na tela, edita visualmente e o Liquid grava apenas as linhas modificadas no arquivo em disco (com supressão de eco).
* **Entregáveis:**
  - Parser de AST semântico decompondo o Markdown em blocos de interface ricos.
  - Motor de gravação cirúrgica linha-a-linha no disco (Surgical Line Patcher).
  - Algoritmo de Supressão de Eco (*Echo Suppression*) e suporte a atalhos (`Cmd+Z`, `/`).

---

### 🔹 Fase 5: v0.5.0 — The Protocol (A Conexão com Agentes e Modelos)
* **Objetivo:** Habilitar a comunicação padronizada com agentes de IA e múltiplos modelos de linguagem.
* **Feature:** **`FEAT-05: ACP & LiteLLM Gateway`**
* **Jornada do Usuário:** O usuário configura seu provedor de IA (Ollama local ou Claude/OpenAI em nuvem) e testa o handshake de agentes via Agent Client Protocol (ACP).
* **Entregáveis:**
  - Servidor JSON-RPC 2.0 ACP sobre WebSocket e stdio.
  - Gateway universal de modelos via LiteLLM com teste de ping e latência.
  - Modal de configurações de IA construído com Astryx.

---

### 🔹 Fase 6: v0.6.0 — The Engine (A Fábrica Autônoma com Podman)
* **Objetivo:** Fechar o ciclo do SCPE compilando especificações aprovadas em código testado automaticamente.
* **Feature:** **`FEAT-06: Podman Sandbox Runner`**
* **Jornada do Usuário:** Ao promover uma feature para `Ready`, o Liquid aciona um agente autônomo que sobe um container Podman isolado, executa tarefas de `tasks.md` em TDD, gera código em `apps/` e altera o estado para `Done`.
* **Entregáveis:**
  - Executor de contêineres efémeros e isolados com Podman rootless (sem docker-compose).
  - Pipeline de execução TDD com agente revisor de qualidade.
  - Streaming de logs de execução e terminal ao vivo para a interface.

---

## 3. Matriz de Rastreabilidade

| Versão | Fase | Nome | Feature Alvo | Status |
| :---: | :---: | :--- | :--- | :---: |
| **`v0.1.0`** | **Fase 1** | **The Reader** | `01-workspace-inspector` | `Draft` 📝 |
| **`v0.2.0`** | **Fase 2** | **The Auditor** | `02-spec-status-and-drift` | `Draft` 📝 |
| **`v0.3.0`** | **Fase 3** | **The Synchronizer** | `03-live-file-sync` | `Draft` 📝 |
| **`v0.4.0`** | **Fase 4** | **The Studio** | `04-wysiwyg-document-editor` | `Draft` 📝 |
| **`v0.5.0`** | **Fase 5** | **The Protocol** | `05-acp-agent-handshake` | `Draft` 📝 |
| **`v0.6.0`** | **Fase 6** | **The Engine** | `06-podman-sandbox-execution` | `Draft` 📝 |
