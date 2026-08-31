# 📐 Plano Técnico — Spec Drift Validator

**Épico:** `EPIC-02` da Feature `FEAT-02`  
**Status:** `Draft` 📝  

---

## 1. Arquitetura e Decisões Técnicas
- Implementação estrita com Rust, React 19, Astryx e Zustand.
- Respeito aos limites do Bounded Context da feature `FEAT-02`.
- Validador consome o Perfil de Metodologia exposto por `/api/workspace/methodology` (`FEAT-01 / EPIC-03`); na ausência de fonte configurada, aplica as regras nativas do SCPE v0.3.0 como fallback.

## 2. Contratos & Interfaces
Contratos orientados por tipos seguros e comunicação via JSON / ACP.
