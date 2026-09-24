use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathGuardError {
    TraversalDetected,
    NotFound,
    Io(String),
}

impl std::fmt::Display for PathGuardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TraversalDetected => write!(f, "Path traversal attempt detected"),
            Self::NotFound => write!(f, "File not found"),
            Self::Io(msg) => write!(f, "I/O error: {msg}"),
        }
    }
}

impl std::error::Error for PathGuardError {}

pub struct PathGuard;

impl PathGuard {
    /// Validates that requested_path relative to root does not escape root.
    /// Rejects any path containing parent components (`..`) or resolving outside root.
    pub fn resolve_safe_path(root: &Path, requested_path: &str) -> Result<PathBuf, PathGuardError> {
        let requested = Path::new(requested_path);

        // Check for relative traversal components (e.g. `..`)
        for component in requested.components() {
            if matches!(component, Component::ParentDir) {
                return Err(PathGuardError::TraversalDetected);
            }
        }

        // Canonicalize root to normalize symlinks and relative references
        let canonical_root = match root.canonicalize() {
            Ok(c) => c,
            Err(_) => {
                // If root doesn't exist yet or can't be canonicalized directly
                root.to_path_buf()
            }
        };

        // Normalize requested path by stripping leading slashes
        let clean_requested = requested_path.trim_start_matches('/');
        let combined = canonical_root.join(clean_requested);

        // If the path exists on disk, check its canonical path
        if combined.exists() {
            if let Ok(canonical_combined) = combined.canonicalize() {
                if !canonical_combined.starts_with(&canonical_root) {
                    return Err(PathGuardError::TraversalDetected);
                }
                return Ok(canonical_combined);
            }
        }

        // If path doesn't exist yet (e.g. for write operations)
        if !combined.starts_with(&canonical_root) {
            return Err(PathGuardError::TraversalDetected);
        }

        Ok(combined)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<FileNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}

/// Recursively scans workspace root directory, excluding hidden dirs and build artifacts (R2).
pub fn scan_workspace_tree(root: &Path) -> Result<FileNode, std::io::Error> {
    scan_node(root, root)
}

fn scan_node(root: &Path, current: &Path) -> Result<FileNode, std::io::Error> {
    let name = if current == root {
        current
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("root")
            .to_string()
    } else {
        current
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string()
    };

    let relative_path = match current.strip_prefix(root) {
        Ok(p) => {
            let s = p.to_string_lossy().to_string();
            if s.is_empty() {
                "/".to_string()
            } else {
                format!("/{}", s.trim_start_matches('/'))
            }
        }
        Err(_) => "/".to_string(),
    };

    if current.is_dir() {
        let mut children = Vec::new();
        if let Ok(entries) = fs::read_dir(current) {
            let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            entries.sort_by_key(|e| e.file_name());

            for entry in entries {
                let child_path = entry.path();
                let child_name = entry.file_name().to_string_lossy().to_string();

                // Exclude hidden dirs and build artifacts (R2)
                if child_name.starts_with('.')
                    || child_name == "target"
                    || child_name == "node_modules"
                    || child_name == "dist"
                {
                    continue;
                }

                if let Ok(child_node) = scan_node(root, &child_path) {
                    children.push(child_node);
                }
            }
        }
        Ok(FileNode {
            name,
            path: relative_path,
            is_dir: true,
            children: Some(children),
            size: None,
        })
    } else {
        let size = current.metadata().map(|m| m.len()).ok();
        Ok(FileNode {
            name,
            path: relative_path,
            is_dir: false,
            children: None,
            size,
        })
    }
}

/// Reads file contents safely within workspace root.
pub fn read_workspace_file(root: &Path, requested_path: &str) -> Result<String, PathGuardError> {
    let safe_path = PathGuard::resolve_safe_path(root, requested_path)?;
    if !safe_path.exists() || !safe_path.is_file() {
        return Err(PathGuardError::NotFound);
    }
    fs::read_to_string(&safe_path).map_err(|e| PathGuardError::Io(e.to_string()))
}

/// Writes file contents safely within workspace root, creating parent directories if needed.
pub fn write_workspace_file(
    root: &Path,
    requested_path: &str,
    content: &str,
) -> Result<PathBuf, PathGuardError> {
    let safe_path = PathGuard::resolve_safe_path(root, requested_path)?;
    if let Some(parent) = safe_path.parent() {
        fs::create_dir_all(parent).map_err(|e| PathGuardError::Io(e.to_string()))?;
    }
    fs::write(&safe_path, content).map_err(|e| PathGuardError::Io(e.to_string()))?;
    Ok(safe_path)
}
