use std::env;
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

const SYSTEM_MARKER: &str = "system.meta";
const SYSTEM_MARKER_CONTENT: &str = "schema=1\n";

#[derive(Debug)]
pub struct Workspace {
    root: PathBuf,
    remove_on_drop: bool,
}

impl Workspace {
    pub fn initialize(path: impl AsRef<Path>) -> Result<Self, WorkspaceError> {
        let requested_root = path.as_ref();

        if !requested_root.is_absolute() {
            return Err(WorkspaceError::unsafe_path(
                requested_root,
                "workspace root must be an absolute path",
            ));
        }

        reject_symlink_components(requested_root)?;
        fs::create_dir_all(requested_root).map_err(|source| {
            WorkspaceError::io("create workspace root", requested_root, source)
        })?;

        let root = requested_root.canonicalize().map_err(|source| {
            WorkspaceError::io("canonicalize workspace root", requested_root, source)
        })?;

        if !root.is_dir() {
            return Err(WorkspaceError::unsafe_path(
                &root,
                "workspace root must be a directory",
            ));
        }

        let workspace = Self {
            root,
            remove_on_drop: false,
        };
        workspace.ensure_system_marker()?;
        Ok(workspace)
    }

    pub fn temporary() -> Result<Self, WorkspaceError> {
        let temporary_parent = env::temp_dir().canonicalize().map_err(|source| {
            WorkspaceError::io("canonicalize temporary directory", env::temp_dir(), source)
        })?;

        for attempt in 0..100_u32 {
            let candidate = temporary_parent.join(temporary_name(attempt));

            match fs::create_dir(&candidate) {
                Ok(()) => {
                    let mut workspace = Self::initialize(&candidate)?;
                    workspace.remove_on_drop = true;
                    return Ok(workspace);
                }
                Err(source) if source.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(source) => {
                    return Err(WorkspaceError::io(
                        "create temporary workspace",
                        candidate,
                        source,
                    ));
                }
            }
        }

        Err(WorkspaceError::unsafe_path(
            temporary_parent,
            "could not allocate a unique temporary workspace",
        ))
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn resolve(&self, relative: impl AsRef<Path>) -> Result<PathBuf, WorkspaceError> {
        let relative = relative.as_ref();
        let normalized = normalize_relative_path(relative)?;
        let candidate = self.root.join(normalized);

        reject_symlink_components_between(&self.root, &candidate)?;

        if !candidate.starts_with(&self.root) {
            return Err(WorkspaceError::unsafe_path(
                relative,
                "resolved path escapes the workspace",
            ));
        }

        Ok(candidate)
    }

    pub fn create_directory(&self, relative: impl AsRef<Path>) -> Result<PathBuf, WorkspaceError> {
        let directory = self.resolve(relative)?;
        fs::create_dir_all(&directory).map_err(|source| {
            WorkspaceError::io("create workspace directory", &directory, source)
        })?;

        let canonical = directory.canonicalize().map_err(|source| {
            WorkspaceError::io("canonicalize workspace directory", &directory, source)
        })?;

        if !canonical.starts_with(&self.root) {
            return Err(WorkspaceError::unsafe_path(
                directory,
                "created directory escapes the workspace",
            ));
        }

        Ok(canonical)
    }

    fn ensure_system_marker(&self) -> Result<(), WorkspaceError> {
        let marker = self.resolve(SYSTEM_MARKER)?;

        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&marker)
        {
            Ok(mut file) => file
                .write_all(SYSTEM_MARKER_CONTENT.as_bytes())
                .map_err(|source| WorkspaceError::io("write system marker", marker, source)),
            Err(source) if source.kind() == io::ErrorKind::AlreadyExists => {
                let metadata = fs::symlink_metadata(&marker).map_err(|source| {
                    WorkspaceError::io("inspect system marker", &marker, source)
                })?;

                if metadata.file_type().is_symlink() || !metadata.is_file() {
                    return Err(WorkspaceError::unsafe_path(
                        marker,
                        "system marker must be a regular file",
                    ));
                }

                Ok(())
            }
            Err(source) => Err(WorkspaceError::io("create system marker", marker, source)),
        }
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        if self.remove_on_drop {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

#[derive(Debug)]
pub enum WorkspaceError {
    UnsafePath {
        path: PathBuf,
        reason: &'static str,
    },
    Io {
        operation: &'static str,
        path: PathBuf,
        source: io::Error,
    },
}

impl WorkspaceError {
    fn unsafe_path(path: impl Into<PathBuf>, reason: &'static str) -> Self {
        Self::UnsafePath {
            path: path.into(),
            reason,
        }
    }

    fn io(operation: &'static str, path: impl Into<PathBuf>, source: io::Error) -> Self {
        Self::Io {
            operation,
            path: path.into(),
            source,
        }
    }
}

impl fmt::Display for WorkspaceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsafePath { path, reason } => {
                write!(
                    formatter,
                    "unsafe workspace path {}: {reason}",
                    path.display()
                )
            }
            Self::Io {
                operation,
                path,
                source,
            } => write!(
                formatter,
                "could not {operation} {}: {source}",
                path.display()
            ),
        }
    }
}

impl std::error::Error for WorkspaceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::UnsafePath { .. } => None,
            Self::Io { source, .. } => Some(source),
        }
    }
}

fn normalize_relative_path(path: &Path) -> Result<PathBuf, WorkspaceError> {
    if path.as_os_str().is_empty() {
        return Err(WorkspaceError::unsafe_path(path, "path must not be empty"));
    }

    let mut normalized = PathBuf::new();

    for component in path.components() {
        match component {
            Component::Normal(value) => normalized.push(value),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(WorkspaceError::unsafe_path(
                    path,
                    "path must be relative and must not contain parent traversal",
                ));
            }
        }
    }

    if normalized.as_os_str().is_empty() {
        return Err(WorkspaceError::unsafe_path(
            path,
            "path must contain a workspace-relative name",
        ));
    }

    Ok(normalized)
}

fn reject_symlink_components(path: &Path) -> Result<(), WorkspaceError> {
    let mut current = PathBuf::new();

    for component in path.components() {
        current.push(component.as_os_str());
        reject_if_symlink(&current)?;
    }

    Ok(())
}

fn reject_symlink_components_between(root: &Path, candidate: &Path) -> Result<(), WorkspaceError> {
    let relative = candidate.strip_prefix(root).map_err(|_| {
        WorkspaceError::unsafe_path(candidate, "candidate is outside the workspace root")
    })?;
    let mut current = root.to_path_buf();

    for component in relative.components() {
        current.push(component.as_os_str());
        reject_if_symlink(&current)?;
    }

    Ok(())
}

fn reject_if_symlink(path: &Path) -> Result<(), WorkspaceError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(WorkspaceError::unsafe_path(
            path,
            "symbolic links are not allowed in workspace paths",
        )),
        Ok(_) => Ok(()),
        Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(source) => Err(WorkspaceError::io("inspect path component", path, source)),
    }
}

fn temporary_name(attempt: u32) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("rust400-{}-{timestamp}-{attempt}", process::id())
}
