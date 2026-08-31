# 📄 PRD Global — FEAT-02: Spec Status, Drift & Liquid Board (Kanban)

**Feature ID:** `FEAT-02`  
**Nome:** Spec Status, Drift & Liquid Board (Kanban View)  
**Projeto:** Liquid ADE  
**Metodologia:** SCPE v0.3.0  
**Status Global:** `Draft` 📝  

---

## 1. Visão Executiva & Jornada do Usuário

### 1.1. Jornadas Verticais de Ponta a Ponta
> *"Como Product Owner e Tech Lead, eu alterno entre o modo Documento e o modo **Liquid Board (Kanban)**. No Kanban, vejo colunas representando os estados do SCPE (`Draft`, `Ready`, `WIP`, `Done`, `Blocked`, `Stale`) com cards para cada épico/tarefa. Ao arrastar um card de `Draft` para `Ready`, o Liquid atualiza o `quick_status.md` no disco em tempo real."*

### 1.2. Escopo do Bounded Context
Esta feature entrega a visão holística do projeto através de:
1. **Badges de Estado:** Indicadores táteis na árvore lateral.
2. **Liquid Board (Kanban Interativo):** Visualização e transição de estados via cards e colunas do SCPE v0.3.0.
3. **Spec Drift Inspector:** Alerta visual e banner de inconsistência para épicos `Stale`.

---

## 2. Invariantes & Regras Inegociáveis

1. **Kanban Orientado a Arquivos (Zero Banco Oculto):** Cada coluna do Kanban reflete a máquina de estados do SCPE (`Draft`, `Ready`, `WIP`, `Done`, `Blocked`, `Stale`). Mover um card altera diretamente o arquivo `quick_status.md` correspondente no disco.
2. **Componentização Astryx:** Os cards de tarefas e épicos utilizam os componentes de card e badges do Astryx Design System.
3. **Corte Vertical Estrito:** A feature só é considerada `Done` quando a visualização e a transição via drag-and-drop / clique funcionarem de ponta a ponta.

---

## 3. Arquitetura de Épicos

| Épico | Nome | Descrição | Status |
| :--- | :--- | :--- | :---: |
| **`EPIC-01`** | **Status Matrix & Liquid Board (Kanban)** | Leitura de estados e painel visual em colunas Kanban com Cards interativos do Astryx. | `Draft` 📝 |
| **`EPIC-02`** | **Spec Drift Validator & State Transitions** | Motor de transição de estados no disco e detecção de inconsistências pós-entrega (`Stale`). | `Draft` 📝 |
