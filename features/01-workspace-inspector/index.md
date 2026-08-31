# 📄 PRD Global — FEAT-01: Workspace Inspector & Spec Viewer

**Feature ID:** `FEAT-01`  
**Nome:** Workspace Inspector & Spec Viewer  
**Projeto:** Liquid ADE  
**Metodologia:** SCPE v0.3.0  
**Status Global:** `Draft` 📝  

---

## 1. Visão Executiva & Jornada do Usuário

### 1.1. Jornada Vertical de Ponta a Ponta
> "Como desenvolvedor ou PO, eu abro o Liquid ADE, vejo a lista de governança e features na barra lateral, e consigo clicar e ler qualquer especificação renderizada no canvas central."

### 1.2. Escopo do Bounded Context
Esta feature representa uma **Fatia Vertical (Vertical Slice)** completa. Ela entrega valor direto para o usuário conectando a Interface (React + Astryx), o Motor (Rust Axum) e o Sistema de Arquivos / Execução.

---

## 2. Invariantes & Regras Inegociáveis

1. **Corte Vertical Estrito:** A feature só é considerada `Done` quando a jornada do usuário funcionar de ponta a ponta.
2. **SSOT no Sistema de Arquivos:** Todo estado é refletido no disco; zero persistência oculta.
3. **Aderência aos Pilares de UI/UX:** Componentes e feedback visual seguem `UI_UX_GUIDELINES.md`.

---

## 3. Arquitetura de Épicos

| Épico | Nome | Descrição | Status |
| :--- | :--- | :--- | :---: |
| **`EPIC-01`** | **Runtime Shell & Astryx Layout** | Servidor Rust (Axum) servindo a SPA estática embutida com o layout de 3 colunas em Astryx. | `Draft` 📝 |
| **`EPIC-02`** | **Workspace FS Scanner & Markdown Viewer** | Scanner em Rust que lê a árvore de diretórios do workspace e endpoint `/api/workspace/tree` consumido pela UI. | `Draft` 📝 |
| **`EPIC-03`** | **Methodology Config Resolver & Adapter** | Tela de configuração (sem terminal) para apontar a metodologia SDD do workspace a uma fonte externa em texto, resolvida em um Perfil de Metodologia usado por scaffolding e validação. | `Draft` 📝 |
