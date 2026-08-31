# 📌 Épico 02: Spec Drift Validator

**Épico:** `EPIC-02` da Feature `FEAT-02`  
**Status:** `Draft` 📝  

---

## 1. Objetivo & Escopo
Algoritmo de auditoria que detecta modificações no `plan.md` de épicos finalizados e marca como `Stale`.

## 2. Critérios de Aceite
- [ ] Quando uma metodologia externa estiver configurada (`FEAT-01 / EPIC-03`), a validação de estrutura e drift usa o Perfil de Metodologia resolvido em vez da máquina de estados fixa do SCPE.
- [ ] Na ausência de configuração, a validação segue as regras nativas do SCPE v0.3.0 (fallback).
- [ ] Implementação de ponta a ponta validada por testes.
- [ ] Zero impacto de performance no runtime do Liquid.
