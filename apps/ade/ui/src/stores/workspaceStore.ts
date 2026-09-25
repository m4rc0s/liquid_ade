import { create } from 'zustand';
import {
  fetchWorkspaceFile,
  fetchWorkspaceTree,
  isRenderableDocument,
  type FileContent,
  type FileNode,
} from '../features/workspace/api';

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

export interface WorkspaceState {
  activeProject: string;
  tree: FileNode | null;
  isTreeLoading: boolean;
  treeError: string | null;
  activePath: string | null;
  activeDocument: FileContent | null;
  isDocumentLoading: boolean;
  documentError: string | null;

  setActiveProject: (name: string) => void;
  setActivePath: (path: string | null) => void;
  setDocumentError: (error: string | null) => void;
  openDocument: (path: string) => Promise<void>;
  loadTree: () => Promise<void>;
  selectNode: (node: FileNode) => void;
}

export const useWorkspaceStore = create<WorkspaceState>((set, get) => ({
  activeProject: 'liquid_ade',
  tree: null,
  isTreeLoading: true,
  treeError: null,
  activePath: null,
  activeDocument: null,
  isDocumentLoading: false,
  documentError: null,

  setActiveProject: (name: string) => set({ activeProject: name }),
  setActivePath: (path: string | null) => set({ activePath: path }),
  setDocumentError: (error: string | null) => set({ documentError: error }),

  openDocument: async (path: string) => {
    set({ activePath: path, isDocumentLoading: true, documentError: null });
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
      if (entry) {
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
