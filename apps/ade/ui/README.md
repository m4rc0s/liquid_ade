# ✨ ade_ui — Liquid ADE Frontend

<div align="center">

### *The High-Density, Document-as-UI Studio for Autonomous Engineering*
**React 19 • TypeScript • Vite • Astryx Design System • Zustand**

</div>

---

## 📖 Visão Geral

O **`ade_ui`** é o studio visual de alta performance do **Liquid ADE**. Inspirado nos padrões de interação e design refinado de ferramentas como **Linear** e **Notion**, ele transforma especificações Markdown vivas em um editor WYSIWYG estruturado em blocos semânticos com feedback tátil e sincronização em tempo real.

O frontend foi desenhado para ser compilado e embutido diretamente no binário único de alta performance do backend em Rust (`ade_core`) através de `rust-embed`, sem depender de Node.js/Bun em ambiente de produção.

---

## 🎨 Astryx Design System & Filosofia de UI/UX

A interface é regida pela constituição descrita em [UI/UX Guidelines](UI_UX_GUIDELINES.md):

### 🏛️ Os 7 Pilares de UI
1. **Tipografia Linear-like:** Hierarquia escaneável com peso marcante (`font-semibold`), corpo leve com espaçamento generoso e fonte Monospace para metadados e tags SCPE.
2. **Cores & Dark Mode Sofisticado:** Camadas de cinza profundo e bordas ultra sutis (`1px solid rgba(255, 255, 255, 0.06)`). Badges semânticas para estados SCPE (`Draft`, `Ready`, `WIP`, `Done`, `Blocked`).
3. **Componentização Astryx:** Componentes táteis com cantos suaves (6px a 8px) e transições imperceptíveis (100ms).
4. **Layout & Densidade Linear:** Grid modular de 4px/8px com respiro focado no texto da especificação.
5. **Hierarquia Visual:** O documento no canvas central é o protagonista; menus e sidebars são discretos e colapsáveis.
6. **Iconografia:** Ícones minimalistas (Lucide) com traço fino de 1.5px.
7. **Feedback Tátil:** Micro-indicadores em tempo real para sincronização de disco e conexão ACP.

### 🧭 Aceleradores & Heurísticas de UX
* **`Cmd+K` / `Ctrl+K` (Command Palette):** Navegação instantânea entre features, busca de épicos e ações rápidas.
* **Menu Slash (`/`):** Inserção rápida de blocos de critérios de aceite, tarefas e seções no editor.
* **Undo / Redo Completo (`Cmd+Z` / `Cmd+Shift+Z`):** Controle total de histórico local de edição.

---

## 🛠️ Stack Tecnológica

| Camada | Tecnologia | Propósito |
| :--- | :--- | :--- |
| **Framework** | [React 19](https://react.dev/) | Renderização reativa e concorrente de alta performance |
| **Linguagem** | [TypeScript 5+](https://www.typescriptlang.org/) | Tipagem estrita ponta a ponta |
| **Bundler / HMR** | [Vite 6+](https://vite.dev/) | Build ultrarrápido com Hot Module Replacement |
| **Design System** | [Astryx Design System](https://github.com/m4rc0s/astryx) / Tailwind CSS | Componentes de alta densidade e estilização utilitária |
| **Estado Global** | [Zustand](https://zustand-demo.pmnd.rs/) | Gerenciamento de estado leve e desacoplado |
| **Protocolo** | ACP (Agent Client Protocol) / JSON-RPC 2.0 | Comunicação em tempo real via WebSocket com o `ade_core` |
| **Linter** | [Oxlint](https://oxc.rs/) | Linter estático em Rust de velocidade extrema |

---

## 📂 Estrutura de Diretórios

```text
ui/
├── public/                # Favicons, SVGs e assets públicos
├── src/
│   ├── assets/            # Imagens e ilustrações da aplicação
│   ├── components/        # Componentes reutilizáveis (Astryx & shadcn/ui customizados)
│   ├── features/          # Fatias verticais de funcionalidade (Package-by-Feature)
│   │   ├── workspace/     # Leitor do sistema de arquivos e indexador de specs
│   │   ├── editor/        # Editor WYSIWYG Document-as-UI e AST blocks
│   │   ├── status-matrix/ # Auditor da máquina de estados e detecção de drift
│   │   └── protocol/      # Painel de controle ACP e orquestrador de agentes
│   ├── services/          # Clientes WebSocket / ACP e API REST
│   ├── stores/            # Stores globais Zustand (workspace, editor, acp)
│   ├── App.tsx            # Shell principal da interface
│   ├── main.tsx           # Entrypoint do React 19
│   └── index.css          # Design tokens e variáveis de tema
├── package.json           # Dependências e scripts
├── tsconfig.json          # Configuração do compilador TypeScript
└── vite.config.ts         # Configurações do Vite
```

---

## 🚀 Scripts Disponíveis

Utilize o [Bun](https://bun.sh/) para gerenciar dependências e executar tarefas:

### Desenvolvimento
```bash
# Iniciar o servidor local de desenvolvimento com Hot Reload
bun run dev
```
Abre a interface em [http://localhost:5173](http://localhost:5173).

### Análise Estática & Lint
```bash
# Executar verificação estática com Oxlint
bun run lint
```

### Testes
```bash
# Executar a suíte de testes unitários e de integração
bun test
```

### Compilação para Produção
```bash
# Validação de tipos (tsc) e compilação do bundle estático em `dist/`
bun run build
```
*Nota: O script de compilação em Rust (`build.rs` do `apps/ade`) executa este comando automaticamente para embutir a UI dentro do binário final do Liquid ADE.*

### Pré-visualização do Bundle
```bash
# Visualizar o build estático gerado em `dist/`
bun run preview
```

---

## 📚 Documentação Relacionada

* [UI/UX Guidelines (Princípios de Design)](UI_UX_GUIDELINES.md)
* [Manifesto Técnico da Aplicação](../app_manifest.md)
* [README Geral do Liquid ADE](../../../README.md)
