# 🎨 Diretrizes e Princípios Fundamentais de UI/UX (Estilo Linear & Notion)

Este documento estabelece os **Princípios Fundamentais de UI e as Heurísticas de UX** para as interfaces do **Liquid ADE**, com inspiração direta no padrão de design e interação do **Linear** e do **Notion**: minimalista, tipografia refinada, foco no documento, alta densidade de informação, atalhos de teclado e micro-interações táteis.

> **Biblioteca Base:** [Astryx Design System](https://github.com/m4rc0s/astryx) (`@astryxdesign/core` - React 19+)

---

## 🏛️ PARTE 1: Os 7 Pilares de UI (Estilo Linear / Notion)

1. **Tipografia (Linear-like):**
   - Hierarquia limpa e escaneável: títulos refinados com peso marcante (`font-semibold`), corpo de texto leve com espaçamento generoso (`leading-relaxed`), e fonte Monospace estrita para metadados, identificadores e tags.
2. **Cores & Superfícies (Dark Mode Sofisticado):**
   - Camadas de cinzas escuros profundos e bordas ultra sutis (`1px solid rgba(255,255,255,0.06)`).
   - Cores semânticas refinadas para status: `Done` (Verde esmeralda), `Ready` (Azul elétrico), `WIP` (Âmbar/Laranja), `Blocked` (Vermelho), `Draft` (Cinza neutro).
3. **Componentização (Astryx):**
   - Botões, menus de contexto, modais e badges estilizados com cantos suaves (6px a 8px) e transições imperceptíveis (100ms).
4. **Layout & Densidade (Linear Style):**
   - Grid modular de 4px/8px. Divisores sutis, sem bordas pesadas ou sombras artificiais. Respiro visual focado no texto da especificação.
5. **Hierarquia Visual:**
   - O documento ativo no canvas é a peça central da tela. Menus e barras laterais ficam discretos e colapsáveis.
6. **Iconografia:**
   - Ícones minimalistas e refinados (Lucide) com traço de 1.5px e escala coerente.
7. **Feedback & Micro-Interações:**
   - Estados visuais táteis em hover, foco com anel sutil e indicadores instantâneos de sincronização ("*Salvo*").

---

## 🧭 PARTE 2: As 8 Diretrizes de UX (Interação Notion/Linear)

1. **Visibilidade do Status:** Indicadores em tempo real de conexão ACP e micro-indicador de salvamento no disco.
2. **Correspondência com o Mundo Real:** Linguagem de produto clara ("*Épicos*", "*Tarefas*", "*Critérios de Aceite*").
3. **Controle e Liberdade:** Suporte completo a **Undo/Redo (`Cmd+Z` / `Cmd+Shift+Z`)** no editor de texto.
4. **Consistência:** Barra lateral de navegação SCPE à esquerda, Canvas central do documento e alternância para Liquid Board (Kanban).
5. **Prevenção de Erros:** Validações sutis e bloqueio de transições inválidas de estado.
6. **Redução da Carga Cognitiva:** Interface zen e focada na leitura e escrita sem poluição visual.
7. **Estética Minimalista:** Eliminação de elementos decorativos; foco absoluto no conteúdo da especificação.
8. **Aceleradores (Power Users - Padrão Linear/Notion):**
   - **`Cmd+K` (Command Palette):** Navegação rápida entre features, busca de épicos e comandos de ação.
   - **Menu Slash (`/`):** Inserção rápida de blocos de texto, checklists de critérios e seções dentro do editor.
