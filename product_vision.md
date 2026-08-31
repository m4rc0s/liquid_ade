# 🌊 Liquid: Manifesto & Visão de Produto

**Versão da Metodologia:** SCPE v0.3.0 (Spec-Compiled Product Engineering)  
**Status do Documento:** Ativo / Fonte Única da Verdade (SSOT)  
**Última Atualização:** 2026-08-31  

---

## 1. Visão Executiva & Proposta de Valor

O **Liquid** é uma plataforma e ambiente de engenharia de produto autônoma orientada a agentes (ADE — *Agentic Development Environment*), projetada para unificar concepção de negócios, arquitetura técnica e geração de software em uma única experiência contínua e determinística.

Eliminando o fosso histórico entre os requisitos de negócio e o código-fonte, o Liquid adota o paradigma **Spec-Driven Development (SDD)** sob a metodologia **SCPE v0.3.0** (*Spec-Compiled Product Engineering*), onde:

> **"A Especificação é o Contrato Soberano. O Documento é a Interface (Document-as-UI). Os Agentes Executam com Precisão Matemática. O Código é Apenas a Consequência."**

### 1.1. O Problema: A Ilusão do "Vibe Coding" e o Abismo da Desconexão
Com o advento dos Modelos de Linguagem (LLMs), a indústria de software mergulhou no *vibe coding* — uma geração desgovernada e não estruturada de código que gera dívida técnica imediata, falta de arquitetura e alucinações de contexto. Os problemas fundamentais atuais são:
- **Desconexão de Requisitos:** Requisitos residem em ferramentas isoladas (Jira, Notion, Confluence, Slack), enquanto o código evolui dissociado da intenção original de produto.
- **Perda de Contexto e Alucinação de Agentes:** Agentes de IA operam sem limites rígidos de domínio (*Bounded Contexts*), tentando inferir a arquitetura a partir de bases de código inteiras.
- **Falta de Governança e Rastreabilidade:** Não há garantia de que cada linha de código gerada por um agente atenda a um critério de aceitação formal e mensurável.
- **Lock-in Proprietário e Perda de Privacidade:** Dependência de ecossistemas fechados na nuvem e envio indiscriminado de propriedade intelectual e código sensível para servidores de terceiros.

### 1.2. A Solução Liquid: Spec-First, Local-First e Code as Consequence
O Liquid reestrutura o ciclo de vida do software através de especificações vivas compiláveis em Markdown estruturado, com sincronização bidirecional em tempo real, orquestração de agentes via protocolos abertos e execução estritamente isolada.

### 1.3. Metodologia Configurável (Pluggable SDD)
O SCPE v0.3.0 é a metodologia nativa e o fallback padrão do Liquid, mas não é a única possível. Cada workspace pode declarar uma **fonte de metodologia** própria — um repositório, pasta ou documento em texto plano descrevendo como aquele time trabalha, desde um padrão robusto e multi-documento (como o próprio SCPE) até um conjuntinho simples de regras escrito por uma única pessoa. O Liquid resolve essa fonte em um **Perfil de Metodologia** e passa a usá-lo para validar estrutura, gerar (*scaffold*) novos arquivos de spec e orientar os agentes — sem travar nenhum time no formato nativo do Liquid. **A configuração é feita por uma tela dedicada, nunca por edição manual de arquivo**, já que quem define a metodologia de um time é frequentemente um perfil não-técnico (PO/PM/Designer) sem terminal ou editor de código. Ver `FEAT-01 / EPIC-03` e `FEAT-02 / EPIC-02`.

---

## 2. Personas, Workflows & A Fronteira do Código

O Liquid foi concebido para unir desenvolvedores e profissionais de produto sobre a **mesma fonte da verdade (arquivos Markdown no Git)**, com uma clara separação de responsabilidades em relação ao código-fonte:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          FONTE ÚNICA DA VERDADE                             │
│                 Arquivos Markdown no Git (.md / SCPE v0.3.0)                │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
         ┌─────────────────────────────┴─────────────────────────────┐
         ▼                                                           ▼
┌──────────────────────────────────────┐            ┌──────────────────────────────────────┐
│       PERFIL TÉCNICO (CLI-FIRST)     │            │    PERFIL DE PRODUTO (UI-FIRST)      │
│     (Engenheiro / Tech Lead)         │            │   (Product Owner / PM / Designer)    │
│                                      │            │                                      │
│ • Cria e edita specs via terminal,   │            │ • Escreve e refina especificações no │
│   CLI ou editor (Neovim / VS Code).  │            │   editor visual WYSIWYG (Notion/     │
│ • Usa a UI como Cockpit de Controle: │            │   Linear style).                     │
│   acompanha status, roadmap e o      │            │ • Acompanha o Liquid Board (Kanban)  │
│   Liquid Board de tarefas.           │            │   e aprova o Readiness Gate.         │
│ • Usa a UI pontualmente para edições │            │ • Visualiza o progresso das entregas │
│   rápidas quando conveniente.        │            │   sem precisar abrir terminal.       │
└──────────────────┬───────────────────┘            └──────────────────┬───────────────────┘
                   │                                                   │
                   └───────────────────────────┬───────────────────────┘
                                               ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                 A FRONTEIRA DO CÓDIGO (Pasta `apps/`)                       │
│                                                                             │
│ • O Liquid ADE NÃO tenta ser um editor de código genérico (como o VS Code). │
│ • O código gerado pelos agentes reside fisicamente em `apps/<app_name>/`.   │
│ • Qualquer pessoa (Dev ou PM) que deseje inspecionar ou editar o código     │
│   fonte manualmente abre a pasta `apps/` no seu editor favorito             │
│   (Neovim, VS Code, Cursor, WebStorm).                                      │
└─────────────────────────────────────────────────────────────────────────────┘
```

* **Foco no que importa:** O Liquid foca em ser a melhor ferramenta do mundo para **Especificação, Governança, Rastreabilidade e Orquestração de Agentes**.
* **Zero Lock-in de Editor:** Não forçamos ninguém a usar um editor de código embutido no browser. A pasta `apps/` é código limpo e padrão no disco.

---

## 3. A Arquitetura Unificada: Artefato Único & Modelo SPA (Figma/Linear Style)

O Liquid adota o padrão de **Artefato Único (12-Factor App)** e **SPA Estática Pura (sem SSR)**. A aplicação opera com a máxima fluidez tanto como um **SaaS na Web** quanto como um **App Desktop Nativo (Tauri 2.0)** sem necessidade de abrir navegador:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           LIQUID ADE INTERFACE                              │
│       React 19 + TypeScript + Astryx Design System + Editor WYSIWYG         │
│          (Single Page Application Pura / Estática — Zero SSR)               │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │ (Mesma SPA / Mesmos Componentes)
         ┌─────────────────────────────┴─────────────────────────────┐
         ▼                                                           ▼
┌──────────────────────────────────────┐            ┌──────────────────────────────────────┐
│        DISTRIBUIÇÃO DESKTOP          │            │          DISTRIBUIÇÃO WEB            │
│         (Tauri 2.0 Wrapper)          │            │       (Servidor Axum em Rust)        │
│                                      │            │                                      │
│ • Janela Nativa sem abrir navegador  │            │ • Servido localmente ou na Nuvem     │
│ • Execução 100% Offline e Segura     │            │ • Cloud SaaS (Multi-tenant) via ENV  │
│ • WebView nativa de alto desempenho  │            │ • Roda em qualquer browser moderno   │
└──────────────────┬───────────────────┘            └──────────────────┬───────────────────┘
                   │                                                   │
                   └───────────────────────────┬───────────────────────┘
                                               ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                             CORE ENGINE (RUST)                              │
│       Servidor Axum + Protocolo ACP (JSON-RPC 2.0) + File Watcher           │
├─────────────────────────────────────────────────────────────────────────────┤
│                       LITELLM UNIVERSAL MODEL ROUTING                       │
│     (Suporte transparente a Ollama, Llama.cpp, Claude, GPT-4o, Gemini)      │
├─────────────────────────────────────────────────────────────────────────────┤
│                EXECUÇÃO & SANDBOXING ISOLADO COM PODMAN                     │
│        (Isolamento rootless de tarefas downstream — sem docker-compose)     │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Os 7 Pilares Arquiteturais Inegociáveis

1. **Engine Core em Rust (Axum / Tokio):**
   - Núcleo de alta performance, com consumo de memória mínimo (<50MB RSS em repouso) e inicialização ultrarrápida (<100ms).
   - Atua como servidor web, despachante ACP e orquestrador de arquivos.

2. **Frontend Reativo em SPA Estática (Sem SSR):**
   - Construído com React 19, TypeScript, Vite, Astryx Design System (`@astryxdesign/core`) e Zustand.
   - **Zero SSR (Sem Node.js em produção):** Adota o modelo de ferramentas modernas (Linear/Figma/VS Code Web), gerando assets estáticos leves que rodam nativamente no browser e dentro do Tauri sem overhead de servidores intermediários.

3. **Protocolo Aberto via Agent Client Protocol (ACP):**
   - Toda comunicação entre a interface (Client) e os Agentes/Engine é governada por JSON-RPC 2.0 sobre `stdio` ou `WebSocket`.

4. **Roteamento Agnóstico via LiteLLM:**
   - Alternância fluida e configurável entre inferência local privada (Ollama, Llama.cpp) ou provedores em nuvem (Claude 3.5 Sonnet, GPT-4o, Gemini Pro) em qualquer modo de execução.

5. **Sandboxing Seguro com Podman (Rootless & Daemonless):**
   - Agentes de IA executam compilações e testes de código dentro de contêineres efémeros gerenciados via **Podman**, eliminando daemons com privilégios de root e descartando `docker-compose`.

6. **Orquestração de Tarefas com Just (`Justfile`):**
   - Automação de desenvolvimento e compilação centralizada em um `Justfile` minimalista e determinístico.

7. **Editor WYSIWYG (Document-as-UI):**
   - Especificações em Markdown tratadas como componentes visuais vivos no estilo Notion/Linear com sincronização bidirecional em tempo real e supressão de eco.
