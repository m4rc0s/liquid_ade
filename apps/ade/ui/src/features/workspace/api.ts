/**
 * Workspace Inspector HTTP contracts.
 *
 * The UI is a projection of disk state: every node and every document rendered by the canvas is
 * fetched from the Rust binary, which owns the path guard.
 */

export interface FileNode {
  name: string;
  path: string;
  is_dir: boolean;
  children?: FileNode[];
  size?: number;
}

export interface FileContent {
  path: string;
  content: string;
}

interface ApiError {
  error: string;
  message: string;
}

async function unwrap<T>(response: Response): Promise<T> {
  if (!response.ok) {
    const detail = (await response.json().catch(() => null)) as ApiError | null;
    throw new Error(detail?.message ?? `Request failed with status ${response.status}`);
  }
  return (await response.json()) as T;
}

export async function fetchWorkspaceTree(): Promise<FileNode> {
  return unwrap<FileNode>(await fetch('/api/workspace/tree'));
}

export async function fetchWorkspaceFile(path: string): Promise<FileContent> {
  return unwrap<FileContent>(await fetch(`/api/workspace/file?path=${encodeURIComponent(path)}`));
}

/** Files the document canvas can render as Document-as-UI. */
export function isRenderableDocument(node: FileNode): boolean {
  return (
    !node.is_dir && /\.(md|markdown|txt|toml|json|rs|ts|tsx|css|html|py|yml|yaml)$/i.test(node.name)
  );
}
