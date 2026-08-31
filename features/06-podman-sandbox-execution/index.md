# 📄 PRD Global — FEAT-06: Podman Sandbox & Autonomous TDD Runner

**Feature ID:** `FEAT-06`  
**Nome:** Podman Sandbox & Autonomous TDD Runner  
**Projeto:** Liquid ADE  
**Metodologia:** SCPE v0.3.0  
**Status Global:** `Draft` 📝  

---

## 1. Visão Executiva & Jornada do Usuário

### 1.1. Jornada Vertical de Ponta a Ponta
> "Como Tech Lead, ao aprovar uma feature para 'Ready', eu vejo um agente de IA autônomo subir um container Podman isolado, escrever testes em TDD, gerar o código em `apps/` e marcar como 'Done' com 100% de aprovação."

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
| **`EPIC-01`** | **Podman Rootless Container Runner** | Orquestrador de contêineres efémeros e isolados gerenciados via Podman. | `Draft` 📝 |
| **`EPIC-02`** | **Downstream TDD Execution Pipeline** | Pipeline autônomo que executa a lista de `tasks.md` sequencialmente até validação do Reviewer. | `Draft` 📝 |
