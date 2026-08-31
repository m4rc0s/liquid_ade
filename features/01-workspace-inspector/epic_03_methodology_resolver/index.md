# 📌 Épico 03: Methodology Config Resolver & Adapter

**Épico:** `EPIC-03` da Feature `FEAT-01`  
**Status:** `Draft` 📝

---

## 1. Objetivo & Escopo
Permitir que o workspace declare, em configuração, qual padrão de Spec-Driven Development (SDD) deve seguir — apontando para uma fonte externa baseada em texto (repositório Git, pasta local ou documento único) — e que o Liquid resolva essa fonte em um **Perfil de Metodologia** consumido pelos demais épicos (scaffolding em `FEAT-01` e validação de estrutura/drift em `FEAT-02`).

## 2. Critérios de Aceite
- [ ] Usuário consegue declarar uma fonte de metodologia (path local ou URL de repositório Git) na configuração do workspace/projeto.
- [ ] **A configuração é feita por uma tela dedicada na UI (Astryx), sem exigir edição manual de arquivo** — o público-alvo inclui perfis não-técnicos (PO/PM/Designer) que não abrem terminal nem editor de código.
- [ ] A tela exibe feedback claro do resultado da resolução (fonte válida, metodologia detectada, erro de leitura/conexão, fallback aplicado) em linguagem não-técnica.
- [ ] Liquid resolve a fonte (clone/leitura) e extrai um Perfil de Metodologia interpretável por agentes, mesmo quando a fonte é um conjunto informal e não estruturado de regras.
- [ ] Na ausência de configuração, o SCPE v0.3.0 nativo do Liquid é usado como metodologia padrão (fallback).
- [ ] Implementação de ponta a ponta validada por testes.
- [ ] Zero impacto de performance no runtime do Liquid.
