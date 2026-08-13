use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LinuxDirectoryEntry {
    pub name: String,
    pub kind: &'static str,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LinuxUserSummary {
    pub username: String,
    pub uid: String,
    pub gid: String,
    pub home: String,
    pub shell: String,
}

pub fn display_linux_path(path: &Path) -> String {
    format!(
        "Linux host view: current session directory\nPath: {}\nRust/400 note: this is a Linux filesystem location for read-only learning access, not an OS/400 library.",
        path.display()
    )
}

pub fn list_linux_directory_entries(path: &Path) -> Result<Vec<LinuxDirectoryEntry>, io::Error> {
    let mut entries = fs::read_dir(path)?
        .map(|entry| {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let kind = if file_type.is_dir() {
                "DIR"
            } else if file_type.is_file() {
                "FILE"
            } else if file_type.is_symlink() {
                "SYMLINK"
            } else {
                "OTHER"
            };

            Ok(LinuxDirectoryEntry {
                name: entry.file_name().to_string_lossy().into_owned(),
                kind,
            })
        })
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(entries)
}

pub fn read_linux_user_summaries(path: &Path) -> Result<Vec<LinuxUserSummary>, io::Error> {
    let content = fs::read_to_string(path)?;
    let mut users = Vec::new();

    for line in content.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }

        let fields: Vec<_> = line.split(':').collect();
        if fields.len() < 7 {
            continue;
        }

        users.push(LinuxUserSummary {
            username: fields[0].to_string(),
            uid: fields[2].to_string(),
            gid: fields[3].to_string(),
            home: fields[5].to_string(),
            shell: fields[6].to_string(),
        });
    }

    Ok(users)
}

pub fn read_host_linux_users() -> Result<Vec<LinuxUserSummary>, io::Error> {
    read_linux_user_summaries(Path::new("/etc/passwd"))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use crate::workspace::Workspace;

    use super::{display_linux_path, list_linux_directory_entries, read_linux_user_summaries};

    #[test]
    fn linux_path_mentions_host_view() {
        let rendered = display_linux_path(Path::new("/tmp/demo"));

        assert!(rendered.contains("Linux host view"));
        assert!(rendered.contains("/tmp/demo"));
    }

    #[test]
    fn linux_directory_entries_are_listed_in_name_order() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        fs::create_dir(workspace.root().join("BETA")).expect("directory should be created");
        fs::write(workspace.root().join("ALPHA.txt"), "demo").expect("file should be created");

        let entries =
            list_linux_directory_entries(workspace.root()).expect("entries should be listed");
        let names = entries
            .into_iter()
            .map(|entry| entry.name)
            .collect::<Vec<_>>();

        assert!(names.windows(2).all(|pair| pair[0] <= pair[1]));
        assert!(names.contains(&"ALPHA.txt".to_string()));
        assert!(names.contains(&"BETA".to_string()));
    }

    #[test]
    fn passwd_reader_extracts_basic_linux_user_fields() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let passwd = workspace.root().join("passwd.sample");
        fs::write(
            &passwd,
            "root:x:0:0:root:/root:/bin/bash\nmw:x:1000:1000:MW:/home/mw:/bin/zsh\n",
        )
        .expect("sample passwd should be written");

        let users = read_linux_user_summaries(&passwd).expect("passwd file should parse");

        assert_eq!(users.len(), 2);
        assert_eq!(users[1].username, "mw");
        assert_eq!(users[1].uid, "1000");
        assert_eq!(users[1].home, "/home/mw");
        assert_eq!(users[1].shell, "/bin/zsh");
    }
}
