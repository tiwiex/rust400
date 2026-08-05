use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::workspace::{Workspace, WorkspaceError};

const LIBRARIES_DIRECTORY: &str = "catalog";
const LIBRARIES_FILE: &str = "catalog/libraries.db";
const LIBRARIES_SCHEMA_HEADER: &str = "schema=1";

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LibraryRecord {
    pub name: String,
    pub text: Option<String>,
    pub created_at_epoch_seconds: u64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CreateLibraryError {
    AlreadyExists { library: String },
    InvalidCatalog(String),
    Workspace(String),
}

impl fmt::Display for CreateLibraryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExists { library } => {
                write!(formatter, "Library {library} already exists.")
            }
            Self::InvalidCatalog(message) | Self::Workspace(message) => {
                formatter.write_str(message)
            }
        }
    }
}

impl std::error::Error for CreateLibraryError {}

pub fn create_library(
    workspace: &Workspace,
    library: &str,
    text: Option<&str>,
) -> Result<LibraryRecord, CreateLibraryError> {
    workspace
        .create_directory(LIBRARIES_DIRECTORY)
        .map_err(workspace_error)?;

    let mut catalog = load_catalog(workspace).map_err(CreateLibraryError::InvalidCatalog)?;

    if catalog.iter().any(|record| record.name == library) {
        return Err(CreateLibraryError::AlreadyExists {
            library: library.to_string(),
        });
    }

    let record = LibraryRecord {
        name: library.to_string(),
        text: text.map(ToOwned::to_owned),
        created_at_epoch_seconds: current_epoch_seconds(),
    };

    catalog.push(record.clone());
    save_catalog(workspace, &catalog).map_err(CreateLibraryError::Workspace)?;
    Ok(record)
}

pub fn list_libraries(workspace: &Workspace) -> Result<Vec<LibraryRecord>, String> {
    load_catalog(workspace)
}

pub fn find_library(workspace: &Workspace, library: &str) -> Result<Option<LibraryRecord>, String> {
    Ok(load_catalog(workspace)?
        .into_iter()
        .find(|record| record.name == library))
}

fn load_catalog(workspace: &Workspace) -> Result<Vec<LibraryRecord>, String> {
    let path = workspace
        .resolve(LIBRARIES_FILE)
        .map_err(display_workspace_error)?;

    match fs::read_to_string(&path) {
        Ok(contents) => parse_catalog(&contents),
        Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(source) => Err(format!(
            "Could not read library catalog {}: {source}",
            path.display()
        )),
    }
}

fn save_catalog(workspace: &Workspace, records: &[LibraryRecord]) -> Result<(), String> {
    let path = workspace
        .resolve(LIBRARIES_FILE)
        .map_err(display_workspace_error)?;
    let temporary_path = temporary_catalog_path(&path);
    let serialized = serialize_catalog(records);

    fs::write(&temporary_path, serialized).map_err(|source| {
        format!(
            "Could not write library catalog {}: {source}",
            temporary_path.display()
        )
    })?;

    fs::rename(&temporary_path, &path).map_err(|source| {
        format!(
            "Could not replace library catalog {}: {source}",
            path.display()
        )
    })
}

fn parse_catalog(contents: &str) -> Result<Vec<LibraryRecord>, String> {
    let mut lines = contents.lines();
    let Some(header) = lines.next() else {
        return Ok(Vec::new());
    };

    if header != LIBRARIES_SCHEMA_HEADER {
        return Err(format!(
            "Library catalog schema is unsupported: expected {LIBRARIES_SCHEMA_HEADER}, found {header}"
        ));
    }

    let mut records = Vec::new();

    for (index, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let mut fields = line.split('\t');
        let name = fields
            .next()
            .ok_or_else(|| format!("Library catalog row {} is missing the name.", index + 2))?;
        let text = fields
            .next()
            .ok_or_else(|| format!("Library catalog row {} is missing the text.", index + 2))?;
        let created_at = fields.next().ok_or_else(|| {
            format!(
                "Library catalog row {} is missing the creation timestamp.",
                index + 2
            )
        })?;

        if fields.next().is_some() {
            return Err(format!(
                "Library catalog row {} contains unexpected extra fields.",
                index + 2
            ));
        }

        let created_at_epoch_seconds = created_at.parse::<u64>().map_err(|source| {
            format!(
                "Library catalog row {} has an invalid creation timestamp: {source}",
                index + 2
            )
        })?;

        records.push(LibraryRecord {
            name: unescape_field(name)?,
            text: match unescape_field(text)? {
                value if value.is_empty() => None,
                value => Some(value),
            },
            created_at_epoch_seconds,
        });
    }

    Ok(records)
}

fn serialize_catalog(records: &[LibraryRecord]) -> String {
    let mut output = String::from(LIBRARIES_SCHEMA_HEADER);
    output.push('\n');

    for record in records {
        output.push_str(&escape_field(&record.name));
        output.push('\t');
        output.push_str(&escape_field(record.text.as_deref().unwrap_or("")));
        output.push('\t');
        output.push_str(&record.created_at_epoch_seconds.to_string());
        output.push('\n');
    }

    output
}

fn escape_field(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
}

fn unescape_field(value: &str) -> Result<String, String> {
    let mut output = String::new();
    let mut chars = value.chars();

    while let Some(character) = chars.next() {
        if character != '\\' {
            output.push(character);
            continue;
        }

        let Some(escaped) = chars.next() else {
            return Err("Library catalog contains an unterminated escape sequence.".to_string());
        };

        match escaped {
            '\\' => output.push('\\'),
            't' => output.push('\t'),
            'n' => output.push('\n'),
            other => {
                return Err(format!(
                    "Library catalog contains an unsupported escape sequence: \\{other}"
                ));
            }
        }
    }

    Ok(output)
}

fn temporary_catalog_path(path: &std::path::Path) -> PathBuf {
    let mut temporary = path.to_path_buf();
    temporary.set_extension("tmp");
    temporary
}

fn current_epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_secs()
}

fn workspace_error(error: WorkspaceError) -> CreateLibraryError {
    CreateLibraryError::Workspace(display_workspace_error(error))
}

fn display_workspace_error(error: WorkspaceError) -> String {
    error.to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::workspace::Workspace;

    use super::{
        LIBRARIES_SCHEMA_HEADER, create_library, find_library, list_libraries, parse_catalog,
    };

    #[test]
    fn create_library_persists_a_record_in_the_workspace_catalog() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");

        let record =
            create_library(&workspace, "MYLIB", Some("Learning library")).expect("create library");

        assert_eq!(record.name, "MYLIB");
        assert_eq!(record.text.as_deref(), Some("Learning library"));

        let libraries = list_libraries(&workspace).expect("list libraries");
        assert_eq!(libraries.len(), 1);
        assert_eq!(libraries[0].name, "MYLIB");
    }

    #[test]
    fn create_library_rejects_duplicates() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");

        create_library(&workspace, "MYLIB", None).expect("first create should succeed");
        let error = create_library(&workspace, "MYLIB", None).expect_err("duplicate should fail");

        assert_eq!(error.to_string(), "Library MYLIB already exists.");
    }

    #[test]
    fn parse_catalog_rejects_unsupported_schema_headers() {
        let error = parse_catalog("schema=9\nMYLIB\t\t1\n").expect_err("schema should fail");
        assert!(error.contains(LIBRARIES_SCHEMA_HEADER));
    }

    #[test]
    fn list_libraries_reopens_persisted_records_after_restart() {
        let test_root =
            std::env::temp_dir().join(format!("rust400-libraries-reopen-{}", std::process::id()));
        let _ = fs::remove_dir_all(&test_root);

        let workspace = Workspace::initialize(&test_root).expect("workspace should initialize");
        create_library(&workspace, "MYLIB", Some("Learning library"))
            .expect("library should be created");
        let workspace_root = workspace.root().to_path_buf();
        drop(workspace);

        let reopened = Workspace::initialize(&workspace_root).expect("workspace should reopen");
        let libraries = list_libraries(&reopened).expect("list libraries");

        assert_eq!(libraries.len(), 1);
        assert_eq!(libraries[0].name, "MYLIB");
        assert_eq!(libraries[0].text.as_deref(), Some("Learning library"));

        fs::remove_dir_all(&workspace_root).expect("test workspace should be removed");
    }

    #[test]
    fn find_library_returns_the_requested_record() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        create_library(&workspace, "MYLIB", Some("Learning library"))
            .expect("library should be created");

        let library = find_library(&workspace, "MYLIB")
            .expect("lookup should succeed")
            .expect("library should exist");

        assert_eq!(library.name, "MYLIB");
        assert_eq!(library.text.as_deref(), Some("Learning library"));
    }
}
