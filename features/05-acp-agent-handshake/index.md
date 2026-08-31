# 📄 PRD Global — FEAT-05: ACP Protocol & LiteLLM Model Gateway

**Feature ID:** `FEAT-05`  
**Nome:** ACP Protocol & LiteLLM Model Gateway  
**Projeto:** Liquid ADE  
**Metodologia:** SCPE v0.3.0  
**Status Global:** `Draft` 📝  

---

## 1. Visão Executiva & Jornada do Usuário

### 1.1. Jornada Vertical de Ponta a Ponta
> "Como desenvolvedor, eu configuro meu modelo de IA favorito (Ollama local ou Claude/OpenAI em nuvem) nas configurações, e testo a conexão enviando comandos padronizados via Agent Client Protocol (ACP)."

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
| **`EPIC-01`** | **ACP JSON-RPC Server** | Implementação do protocolo ACP baseado em JSON-RPC 2.0 sobre WebSocket e stdio. | `Draft` 📝 |
| **`EPIC-02`** | **LiteLLM Flexible Router** | Gateway universal de modelos conectando o servidor ACP a Ollama ou APIs de nuvem. | `Draft` 📝 |
