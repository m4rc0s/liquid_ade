# 📐 Plano Técnico — Methodology Config Resolver & Adapter

**Épico:** `EPIC-03` da Feature `FEAT-01`  
**Status:** `Draft` 📝

---

## 1. Arquitetura e Decisões Técnicas
- Implementação estrita com Rust, React 19, Astryx e Zustand.
- Respeito aos limites do Bounded Context da feature `FEAT-01`.
- Novo campo `methodology.source` na configuração do workspace, aceitando:
  - Caminho local (pasta ou arquivo único dentro do próprio repositório).
  - URL de repositório Git externo (clonado/atualizado em cache local read-only).
- A metodologia é sempre tratada como **texto plano interpretado por agente**, nunca como schema binário fixo — cobre desde um padrão robusto e multi-arquivo (como o próprio SCPE v0.3.0) até um único arquivo de regras informal.
- O resultado da resolução é um **Perfil de Metodologia** (estrutura esperada de pastas/arquivos, convenções de nomenclatura e, quando existir, máquina de estados) cacheado e exposto para as demais features consumirem.
- **Canal primário de configuração é a UI, não o arquivo:** a tela grava o `methodology.source` no disco por baixo dos panos; a edição manual do arquivo de configuração continua possível (perfil técnico), mas nunca é pré-requisito para o fluxo de um usuário não-técnico.

## 2. Contratos & Interfaces
- Contratos orientados por tipos seguros e comunicação via JSON / ACP.
- Endpoint `/api/workspace/methodology` (GET) expõe a fonte configurada, o status da resolução (`resolved`, `unresolved`, `fallback-scpe`) e o Perfil de Metodologia interpretado, consumido pela UI e por `FEAT-02 / EPIC-02`.
- Endpoint `/api/workspace/methodology` (PUT) grava a nova fonte a partir da tela de configuração e dispara a resolução de forma síncrona, devolvendo o resultado (sucesso, erro legível ou fallback) para feedback imediato na UI.
- Tela de Configuração de Metodologia (modal Astryx, mesmo padrão do modal de configurações de IA da `FEAT-05`): campo único para path/URL, botão "Detectar", preview do Perfil de Metodologia resolvido e opção de reverter para o SCPE nativo.
