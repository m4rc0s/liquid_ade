# 📖 Glossário de Domínio e Linguagem Ubíqua

**Metodologia:** SCPE v0.3.0 (Spec-Compiled Product Engineering)  
**Status:** Canônico e Obrigatório / Fonte Única da Verdade  
**Última Atualização:** 2026-08-31  

Este glossário define a **Linguagem Ubíqua** do ecossistema **Liquid**. Todos os documentos de especificação, manifestos, modelos de dados e agentes autônomos devem utilizar estritamente os termos aqui definidos para eliminar ambiguidades conceituais e alucinações de contexto.

---

## 1. Conceitos Fundamentais do SCPE v0.3.0

### SCPE (Spec-Compiled Product Engineering)
Metodologia de engenharia de software onde especificações em linguagem natural e modelos conceituais estruturados servem como o código-fonte primário compilável. Em SCPE, o código executável é um subproduto determinístico gerado, validado e mantido por agentes de inteligência artificial a partir de documentos vivos.

### SDD (Spec-Driven Development)
Processo de desenvolvimento centrado na escrita e refinamento de especificações rigorosas antes de qualquer geração de código. Rejeita o *vibe coding* desordenado e foca na intenção pura de negócio e nas fronteiras de domínio.

### Metodologia Configurável (Pluggable Methodology)
Capacidade do Liquid de operar sob um padrão de Spec-Driven Development diferente do SCPE v0.3.0 nativo. O workspace aponta para uma **fonte de metodologia** externa em texto plano (um repositório, uma pasta ou um documento único) — que pode ser tão estruturada quanto o próprio SCPE ou tão simples quanto um conjuntinho informal de regras escrito por uma única pessoa. A configuração é feita por uma tela dedicada na UI, nunca exigindo edição manual de arquivo, pois quem define a metodologia de um time é frequentemente um perfil não-técnico (PO/PM/Designer). Na ausência de configuração, o SCPE v0.3.0 é usado como padrão (fallback).

### Perfil de Metodologia (Methodology Profile)
Representação interpretada pelo Liquid a partir de uma fonte de metodologia configurada: estrutura esperada de pastas/arquivos, convenções de nomenclatura e, quando existir, máquina de estados. É consumido pelos agentes para validar estrutura (Spec Drift) e para gerar (scaffold) novos arquivos de spec conforme o padrão daquele workspace, em vez do formato nativo do Liquid.

### Code as Consequence (Código como Consequência)
Princípio arquitetural no qual o código-fonte físico (localizado na pasta `apps/`) nunca é escrito diretamente por humanos em modo cascata, mas sim compilado e materializado como consequência direta de uma especificação formal aprovada.

### SSOT (Single Source of Truth / Fonte Única da Verdade)
Garantia arquitetural de que o estado real do projeto reside estritamente nos arquivos de texto em disco versionados no Git (`.md`, `.yaml`, `.rs`, etc.). A interface do usuário (UI) não possui banco de dados proprietário oculto; ela é uma projeção reativa em tempo real do sistema de arquivos.

### Upstream (A Onda Subindo)
Fase do ciclo de desenvolvimento voltada à elicitação, validação de regras de negócio, arquitetura conceitual e escrita de especificações. É liderada por humanos (Engenheiros de Negócios, Product Managers, Tech Leads) com auxílio de agentes de análise.

### Downstream (A Onda Quebrando)
Fase do ciclo de desenvolvimento iniciada após uma feature atingir o estado **Ready**. Envolve a distribuição automatizada de tarefas para agentes de IA que implementam código em sandboxes isolados via Podman, escrevem testes unitários (TDD) e atualizam o estado das tarefas no repositório.

---

## 2. Protocolos, Runtime & Ferramentas

### ACP (Agent Client Protocol)
Protocolo padronizado de comunicação cliente-servidor baseado em JSON-RPC 2.0 (transmitido via `stdio` ou `WebSocket`). Define contratos estritos para que o Liquid ADE descubra, envie contexto, ordene execuções, receba streaming de respostas e monitore o ciclo de vida de agentes de IA locais ou remotos de forma totalmente desacoplada.

### SPA Estática Pura (Zero SSR)
Decisão arquitetural do frontend do Liquid ADE: uma aplicação em React 19 + TypeScript compilada pelo Vite que gera apenas ativos estáticos (HTML/CSS/JS). Rejeita SSR para eliminar dependências de servidores Node.js em produção e permitir execução idêntica no navegador (Web/SaaS) e em janela nativa (Tauri Desktop).

### Podman Sandboxing (Execução Isolada Rootless)
Mecanismo de segurança em que tarefas de código geradas por agentes (instalação de pacotes, compilação, execução de testes e scripts) rodam em contêineres efémeros gerenciados via **Podman** sem daemons de root. Garante isolamento completo do ambiente hospedeiro do usuário sem a complexidade de `docker-compose`.

### Just (Task Runner)
Ferramenta de automação e orquestração de comandos do repositório (`Justfile`), garantindo comandos determinísticos e unificados (`just dev`, `just build`, `just test`) para humanos e agentes.

### Model-Agnostic Routing (LiteLLM)
Camada de abstração e despacho que permite ao Liquid alternar de forma transparente entre inferência 100% local/privada (Ollama, Llama.cpp, vLLM) e provedores em nuvem (Anthropic Claude, OpenAI, Google Gemini), sem alterar a lógica de negócios da plataforma.

---

## 3. Máquina de Estados e Ciclo de Vida (State Machine SCPE v0.3.0)

Toda feature, fase e tarefa no ecossistema Liquid obedece a uma máquina de estados finita estrita:

```
┌─────────┐       Validação       ┌─────────┐     Disparo ACP      ┌─────────┐
│  DRAFT  │ ────────────────────▶ │  READY  │ ───────────────────▶ │   WIP   │
└─────────┘     (Tech Lead / PM)  └─────────┘    (Agente Inicia)   └─────────┘
     ▲                                 │                                │
     │                                 │ Resolução                      │ Conclusão + Testes
     │                                 ▼                                ▼
     │                            ┌─────────┐                      ┌─────────┐
     └─────────────────────────── │ BLOCKED │                      │  DONE   │
          Retorno p/ Upstream     └─────────┘                      └─────────┘
                                                                        │
                                   Edição do plan.md pós-entrega         │
                                  ──────────────────────────────────────┘
                                  ▼
                             ┌─────────┐
                             │  STALE  │  (Força revalidação no Readiness Gate)
                             └─────────┘
```

| Estado | Significado Semântico | Gatilho de Transição |
| :--- | :--- | :--- |
| **`DRAFT`** | Especificação em ideação/construção no Upstream. | Criação de uma nova feature ou épico. |
| **`READY`** | Especificação aprovada no *Readiness Gate*; pronta para downstream. | Validação do modelo conceitual e critérios pelo Tech Lead / PM. |
| **`WIP`** (*Work In Progress*) | Em execução ativa por um agente autônomo. | Agente inicia a primeira tarefa sequencial de `tasks.md` via ACP. |
| **`DONE`** | Implementação concluída com 100% dos testes unitários e critérios aprovados. | Agente Revisor executa suíte de testes em sandbox e aprova a entrega. |
| **`BLOCKED`** | Execução interrompida por erro técnico, dependência ausente ou ambiguidade. | Agente ou desenvolvedor detecta bloqueio intransponível. |
| **`STALE`** | Especificação alterada pós-entrega, exigindo revalidação técnica de drift. | Qualquer commit no `plan.md` de um épico previamente `Done`. |

---

## 4. Document-as-UI & Editor WYSIWYG

### Document-as-UI
Paradigma no qual arquivos Markdown estruturados não são apenas renderizados tipograficamente, mas sim interpretados como uma árvore de componentes de interface ricos (chips de metadados, badges interativos, tabelas de domínio, checklists clicáveis e blocos BDD).

### AST Block Model (Árvore Sintática Abstrata de Blocos)
Modelo de dados em árvore sintática que divide uma especificação em blocos semânticos tipados (`HeaderNode`, `MetaChipsNode`, `RequirementTableNode`, `BddScenarioNode`, `CriteriaListNode`, `TaskItemNode`). Cada nó possui renderização visual específica através do **Astryx Design System** e capacidade de edição bidirecional direta na tela.

### Two-Way Sync (Sincronização Bidirecional com Supressão de Eco)
Motor que sincroniza alterações entre a interface gráfica e o sistema de arquivos local:
1. **Edição na UI:** Altera cirurgicamente as linhas correspondentes no disco sem afetar formatação ou comentários adjacentes.
2. **Edição no Disco (VS Code / Neovim / Git):** O File Watcher detecta a mudança e atualiza a UI reativamente em <50ms.
3. **Echo Suppression (Supressão de Eco):** Mecanismo que ignora eventos de arquivo gerados pela própria UI, evitando loops infinitos de re-renderização e gravação.
