# 📋 TASKS — Methodology Config Resolver & Adapter

**Épico:** `EPIC-03` da Feature `FEAT-01`  
**Status:** `Draft` 📝

---

## 📝 Checklist de Tarefas Atômicas

- [ ] **TASK-01.3.1**: Definir o campo `methodology.source` na configuração de workspace/projeto (path local ou URL de repositório Git).
- [ ] **TASK-01.3.2**: Implementar resolvedor em Rust que localiza/clona a fonte apontada e lê seu conteúdo como texto.
- [ ] **TASK-01.3.3**: Implementar interpretação via agente de IA que extrai um Perfil de Metodologia (estrutura de pastas/arquivos esperada, convenções, máquina de estados quando existir) a partir de fontes tanto formais quanto informais.
- [ ] **TASK-01.3.4**: Aplicar fallback automático para o SCPE v0.3.0 nativo quando nenhuma fonte for configurada ou a resolução falhar.
- [ ] **TASK-01.3.5**: Expor endpoint `/api/workspace/methodology` (GET/PUT) com a fonte, status e Perfil de Metodologia resolvido.
- [ ] **TASK-01.3.6**: Construir a **tela de Configuração de Metodologia em Astryx** (campo de path/URL, botão de detecção, preview do Perfil resolvido e opção de reverter ao SCPE nativo) acessível sem terminal ou editor de código, para uso por perfis não-técnicos (PO/PM/Designer).
- [ ] **TASK-01.3.7**: Utilizar o Perfil de Metodologia resolvido para gerar (*scaffold*) novos arquivos de spec conforme os templates/estrutura da fonte configurada.

---

## ✅ Definition of Done
- [ ] Todas as tarefas concluídas e testadas.
- [ ] Conformidade estrita com o SCPE v0.3.0.
