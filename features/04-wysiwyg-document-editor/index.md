# 📄 PRD Global — FEAT-04: WYSIWYG Document Editor (Document-as-UI)

**Feature ID:** `FEAT-04`  
**Nome:** WYSIWYG Document Editor (Estilo Linear / Notion)  
**Projeto:** Liquid ADE  
**Metodologia:** SCPE v0.3.0  
**Status Global:** `Draft` 📝  

---

## 1. Visão Executiva & Jornada do Usuário

### 1.1. Jornada Vertical de Ponta a Ponta
> *"Como Product Owner, eu abro o editor de especificação no estilo Notion/Linear. Posso digitar naturalmente, usar o menu slash (`/`) para inserir seções e checklists de critérios de aceitação, clicar em chips de status para alterar o estado do épico, e o Liquid grava o patch cirúrgico no arquivo `.md` no disco sem quebrar a formatação original."*

### 1.2. Escopo do Bounded Context
A feature transforma o arquivo Markdown em blocos interativos limpos e elegantes:
1. **Pílulas & Metadados (Header):** Chips táteis para alterar status, IDs e prioridades com um clique.
2. **Checklists de Critérios de Aceite:** Caixas interativas com barra de progresso visual de conclusão.
3. **Menu Slash (`/`):** Inserção fluida de títulos, listas e callouts no estilo Notion.
4. **Surgical Line Patcher:** Gravação direta linha-a-linha no arquivo `.md` com supressão de eco.

---

## 2. Invariantes & Regras Inegociáveis

1. **Zero Data Loss:** 100% do conteúdo Markdown original é preservado. Trechos genéricos usam o bloco de fallback `RawBlock`.
2. **Patch Cirúrgico:** Edições pontuais (ex: marcar um checkbox) alteram estritamente a linha correspondente no disco.
3. **Estilo Minimalista:** Interface limpa no estilo Linear/Notion, rápida e focada no teclado.

---

## 3. Arquitetura de Épicos

| Épico | Nome | Descrição | Status |
| :--- | :--- | :--- | :---: |
| **`EPIC-01`** | **Notion-Style Semantic Blocks** | Parser semântico e componentes de blocos visuais (chips, checklists, callouts) com Astryx. | `Draft` 📝 |
| **`EPIC-02`** | **Surgical Line Patcher & Echo Suppression** | Motor de gravação cirúrgica no disco com proteção contra loops de atualização e atalhos de teclado. | `Draft` 📝 |
