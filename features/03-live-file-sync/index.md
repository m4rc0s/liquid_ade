# 📄 PRD Global — FEAT-03: Live External File Synchronization

**Feature ID:** `FEAT-03`  
**Nome:** Live External File Synchronization  
**Projeto:** Liquid ADE  
**Metodologia:** SCPE v0.3.0  
**Status Global:** `Draft` 📝  

---

## 1. Visão Executiva & Jornada do Usuário

### 1.1. Jornada Vertical de Ponta a Ponta
> "Como desenvolvedor, quando eu altero um arquivo Markdown no Neovim ou VS Code, a tela do Liquid ADE atualiza instantaneamente (<50ms) sem recarregar a página e sem perder meu ponto de leitura."

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
| **`EPIC-01`** | **Async File Watcher Engine** | Módulo em Rust com `notify` monitorando alterações no sistema de arquivos da pasta `features/`. | `Draft` 📝 |
| **`EPIC-02`** | **WebSocket Delta Streamer** | Canal WebSocket que envia o conteúdo atualizado para a store Zustand na UI. | `Draft` 📝 |
