use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

use rust400::workspace::{Workspace, WorkspaceError};

struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new(label: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after the Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "rust400-test-{label}-{}-{timestamp}",
            process::id()
        ));
        fs::create_dir(&path).expect("test directory should be unique");
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[test]
fn permanent_workspace_contains_all_initialized_state() {
    let test_root = TestDirectory::new("permanent");
    let workspace_path = test_root.path().join("system");

    let workspace = Workspace::initialize(&workspace_path).expect("workspace should initialize");
    let objects = workspace
        .create_directory("state/objects")
        .expect("workspace child should be created");

    assert_eq!(workspace.root(), workspace_path.canonicalize().unwrap());
    assert!(workspace.root().join("system.meta").is_file());
    assert!(objects.starts_with(workspace.root()));
    assert!(objects.is_dir());
}

#[test]
fn parent_traversal_is_rejected_without_outside_write() {
    let test_root = TestDirectory::new("traversal");
    let workspace = Workspace::initialize(test_root.path().join("system"))
        .expect("workspace should initialize");
    let outside = test_root.path().join("escaped");

    let result = workspace.create_directory("../escaped");

    assert!(matches!(result, Err(WorkspaceError::UnsafePath { .. })));
    assert!(!outside.exists());
}

#[test]
fn temporary_workspace_is_removed_when_released() {
    let workspace_path = {
        let workspace = Workspace::temporary().expect("temporary workspace should initialize");
        let path = workspace.root().to_path_buf();
        assert!(path.join("system.meta").is_file());
        path
    };

    assert!(!workspace_path.exists());
}

#[cfg(unix)]
#[test]
fn symlinked_workspace_root_is_rejected_without_outside_write() {
    use std::os::unix::fs::symlink;

    let test_root = TestDirectory::new("root-symlink");
    let outside = test_root.path().join("outside");
    let linked_root = test_root.path().join("linked-system");
    fs::create_dir(&outside).unwrap();
    symlink(&outside, &linked_root).unwrap();

    let result = Workspace::initialize(&linked_root);

    assert!(matches!(result, Err(WorkspaceError::UnsafePath { .. })));
    assert!(!outside.join("system.meta").exists());
}

#[cfg(unix)]
#[test]
fn symlinked_child_is_rejected_without_outside_write() {
    use std::os::unix::fs::symlink;

    let test_root = TestDirectory::new("child-symlink");
    let outside = test_root.path().join("outside");
    fs::create_dir(&outside).unwrap();
    let workspace = Workspace::initialize(test_root.path().join("system"))
        .expect("workspace should initialize");
    symlink(&outside, workspace.root().join("escape")).unwrap();

    let result = workspace.create_directory("escape/written-outside");

    assert!(matches!(result, Err(WorkspaceError::UnsafePath { .. })));
    assert!(!outside.join("written-outside").exists());
}
