import { create } from 'zustand';
import {
  fetchWorkspaceFile,
  fetchWorkspaceTree,
  isRenderableDocument,
  type FileContent,
  type FileNode,
} from '../features/workspace/api';
import {
  fetchScpeOutline,
  fetchScpeEpic,
  type ScpeOutline,
  type EpicDetail,
  type TaskDetail,
} from '../features/scpe/api';

const PREFERRED_ENTRY_DOCUMENT = 'product_vision.md';

function errorMessage(cause: unknown, fallback: string): string {
  return cause instanceof Error ? cause.message : fallback;
}

function findNode(node: FileNode, predicate: (candidate: FileNode) => boolean): FileNode | null {
  if (predicate(node)) {
    return node;
  }
  for (const child of node.children ?? []) {
    const found = findNode(child, predicate);
    if (found) {
      return found;
    }
  }
  return null;
}

export type NavigatorMode = 'specs' | 'files';
export type RightTab = 'copilot' | 'inspector';

export interface WorkspaceState {
  // Global & Navigation Mode
  activeMode: NavigatorMode;
  activeProject: string;

  // File tree state
  tree: FileNode | null;
  isTreeLoading: boolean;
  treeError: string | null;

  // Active file / document state
  activePath: string | null;
  activeDocument: FileContent | null;
  isDocumentLoading: boolean;
  documentError: string | null;

  // SCPE Specs State
  outline: ScpeOutline | null;
  isOutlineLoading: boolean;
  outlineError: string | null;

  activeEpicRef: { feature: string; epic: string } | null;
  activeEpicDetail: EpicDetail | null;
  isEpicLoading: boolean;
  epicError: string | null;

  // Tabs, Filters & Dispatched Actions
  activeRightTab: RightTab;
  selectedPhase: number | null;
  dispatchedTaskPrompt: string | null;

  // Actions
  setActiveMode: (mode: NavigatorMode) => void;
  setActiveProject: (name: string) => void;
  setActivePath: (path: string | null) => void;
  setDocumentError: (error: string | null) => void;
  setActiveRightTab: (tab: RightTab) => void;
  setSelectedPhase: (phase: number | null) => void;

  openDocument: (path: string) => Promise<void>;
  loadTree: () => Promise<void>;
  loadOutline: () => Promise<void>;
  selectNode: (node: FileNode) => void;
  selectEpic: (feature: string, epic: string) => Promise<void>;
  dispatchTaskToCopilot: (task: TaskDetail) => void;
  clearDispatchedTask: () => void;
}

export const useWorkspaceStore = create<WorkspaceState>((set, get) => ({
  activeMode: 'specs',
  activeProject: 'liquid_ade',

  tree: null,
  isTreeLoading: true,
  treeError: null,

  activePath: null,
  activeDocument: null,
  isDocumentLoading: false,
  documentError: null,

  outline: null,
  isOutlineLoading: true,
  outlineError: null,

  activeEpicRef: null,
  activeEpicDetail: null,
  isEpicLoading: false,
  epicError: null,

  activeRightTab: 'copilot',
  selectedPhase: null,
  dispatchedTaskPrompt: null,

  setActiveMode: (mode) => set({ activeMode: mode }),
  setActiveProject: (name) => set({ activeProject: name }),
  setActivePath: (path) => set({ activePath: path }),
  setDocumentError: (error) => set({ documentError: error }),
  setActiveRightTab: (tab) => set({ activeRightTab: tab }),
  setSelectedPhase: (phase) => set({ selectedPhase: phase }),

  dispatchTaskToCopilot: (task) => {
    const prompt = `Please review and help implement ${task.label}: "${task.text}".`;
    set({
      dispatchedTaskPrompt: prompt,
      activeRightTab: 'copilot',
    });
  },

  clearDispatchedTask: () => set({ dispatchedTaskPrompt: null }),

  openDocument: async (path: string) => {
    set({
      activePath: path,
      isDocumentLoading: true,
      documentError: null,
      activeEpicRef: null,
      activeEpicDetail: null,
    });
    try {
      const doc = await fetchWorkspaceFile(path);
      set({ activeDocument: doc, isDocumentLoading: false });
    } catch (cause) {
      set({
        activeDocument: null,
        documentError: errorMessage(cause, `Could not read ${path}`),
        isDocumentLoading: false,
      });
    }
  },

  loadOutline: async () => {
    set({ isOutlineLoading: true, outlineError: null });
    try {
      const data = await fetchScpeOutline();
      set({ outline: data, isOutlineLoading: false });

      // If no epic or document is open, select the first available epic or prefer entry doc
      const current = get();
      if (!current.activeEpicRef && !current.activeDocument) {
        const firstFeature = data.features[0];
        const firstEpic = firstFeature?.epics[0];
        if (firstFeature && firstEpic) {
          void get().selectEpic(firstFeature.slug, firstEpic.slug);
        }
      }
    } catch (cause) {
      set({
        outline: null,
        outlineError: errorMessage(cause, 'Could not load SCPE outline'),
        isOutlineLoading: false,
      });
    }
  },

  selectEpic: async (feature: string, epic: string) => {
    const planPath = `features/${feature}/epics/${epic}/plan.md`;
    set({
      activeEpicRef: { feature, epic },
      activePath: planPath,
      isEpicLoading: true,
      epicError: null,
      activeDocument: null,
      selectedPhase: null,
    });

    try {
      const detail = await fetchScpeEpic(feature, epic);
      set({ activeEpicDetail: detail, isEpicLoading: false });
    } catch (cause) {
      set({
        activeEpicDetail: null,
        epicError: errorMessage(cause, `Could not load epic ${epic}`),
        isEpicLoading: false,
      });
    }
  },

  loadTree: async () => {
    set({ isTreeLoading: true, treeError: null });
    try {
      const root = await fetchWorkspaceTree();
      set({
        tree: root,
        activeProject: root.name || 'workspace',
        isTreeLoading: false,
      });

      const entry = findNode(root, (node) => node.name === PREFERRED_ENTRY_DOCUMENT);
      if (entry && get().activeMode === 'files') {
        void get().openDocument(entry.path);
      }
    } catch (cause) {
      set({
        tree: null,
        treeError: errorMessage(cause, 'Could not scan the workspace'),
        isTreeLoading: false,
      });
    }
  },

  selectNode: (node: FileNode) => {
    if (!isRenderableDocument(node)) {
      set({
        activePath: node.path,
        activeDocument: null,
        documentError: `${node.name} is not a document the canvas can render.`,
      });
      return;
    }
    void get().openDocument(node.path);
  },
}));
