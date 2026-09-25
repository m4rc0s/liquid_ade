import { useState } from 'react';
import type { FileNode } from './api';

interface WorkspaceTreeProps {
  root: FileNode | null;
  activePath: string | null;
  isLoading: boolean;
  error: string | null;
  onSelect: (node: FileNode) => void;
  onRetry: () => void;
}

interface TreeNodeProps {
  node: FileNode;
  depth: number;
  activePath: string | null;
  onSelect: (node: FileNode) => void;
}

/** Directories that start collapsed so the tree stays scannable on a fresh workspace. */
const COLLAPSED_BY_DEFAULT = new Set(['apps', 'docs', 'assets', 'archive']);

function TreeNode({ node, depth, activePath, onSelect }: TreeNodeProps) {
  const [isExpanded, setIsExpanded] = useState(depth < 1 && !COLLAPSED_BY_DEFAULT.has(node.name));

  if (!node.is_dir) {
    const isActive = node.path === activePath;
    return (
      <button
        type="button"
        className={`astryx-tree-item ${isActive ? 'active' : ''}`}
        style={{ paddingLeft: `${8 + depth * 14}px` }}
        onClick={() => onSelect(node)}
        title={node.path}
      >
        <span className="astryx-tree-icon">›</span>
        <span className="astryx-tree-label">{node.name}</span>
      </button>
    );
  }

  const children = node.children ?? [];

  return (
    <div className="astryx-tree-branch">
      <button
        type="button"
        className="astryx-tree-item dir"
        style={{ paddingLeft: `${8 + depth * 14}px` }}
        onClick={() => setIsExpanded((value) => !value)}
        aria-expanded={isExpanded}
      >
        <span className={`astryx-tree-caret ${isExpanded ? 'open' : ''}`}>▸</span>
        <span className="astryx-tree-label">{node.name}</span>
        <span className="astryx-tree-count">{children.length}</span>
      </button>

      {isExpanded &&
        children.map((child) => (
          <TreeNode
            key={child.path}
            node={child}
            depth={depth + 1}
            activePath={activePath}
            onSelect={onSelect}
          />
        ))}
    </div>
  );
}

/** Left navigation column: a live projection of the workspace directory tree. */
export function WorkspaceTree({
  root,
  activePath,
  isLoading,
  error,
  onSelect,
  onRetry,
}: WorkspaceTreeProps) {
  if (isLoading) {
    return <div className="astryx-tree-placeholder">Scanning workspace…</div>;
  }

  if (error) {
    return (
      <div className="astryx-error-box">
        <span>{error}</span>
        <button type="button" className="astryx-btn astryx-btn-secondary" onClick={onRetry}>
          Retry
        </button>
      </div>
    );
  }

  if (!root) {
    return <div className="astryx-tree-placeholder">No workspace loaded.</div>;
  }

  return (
    <div className="astryx-nav-tree">
      <div className="astryx-nav-group-title">
        <span>{root.name || 'workspace'}</span>
      </div>
      {(root.children ?? []).map((child) => (
        <TreeNode
          key={child.path}
          node={child}
          depth={0}
          activePath={activePath}
          onSelect={onSelect}
        />
      ))}
    </div>
  );
}
