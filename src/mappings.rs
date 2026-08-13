#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct LinuxMapping {
    pub term: &'static str,
    pub rust400_concept: &'static str,
    pub rust400_command: &'static str,
    pub linux_analogy: &'static str,
    pub shared_idea: &'static str,
    pub key_difference: &'static str,
    pub linux_example: &'static str,
    pub behavior_label: &'static str,
    pub try_next: &'static [&'static str],
}

const MAPPINGS: &[LinuxMapping] = &[
    LinuxMapping {
        term: "PATH",
        rust400_concept: "Library list",
        rust400_command: "DSPLIBL",
        linux_analogy: "An ordered search path such as PATH.",
        shared_idea: "Search locations are checked in order.",
        key_difference: "PATH searches mainly for executable files. A library list resolves typed objects across libraries.",
        linux_example: "printf '%s\\n' \"$PATH\" | tr ':' '\\n'",
        behavior_label: "Historically inspired",
        try_next: &["WRKLIB", "DSPLIBL", "CRTLIB"],
    },
    LinuxMapping {
        term: "PROCESS",
        rust400_concept: "Job",
        rust400_command: "DSPJOB",
        linux_analogy: "A process plus session metadata.",
        shared_idea: "Both identify active work, its owner, and some execution state.",
        key_difference: "A job carries richer operational context than a single Linux process.",
        linux_example: "ps -ef",
        behavior_label: "Emulated",
        try_next: &["DSPJOB", "DSPUSRPRF"],
    },
    LinuxMapping {
        term: "USER",
        rust400_concept: "User profile",
        rust400_command: "DSPUSRPRF",
        linux_analogy: "A Linux user account.",
        shared_idea: "Both represent an identity associated with work and permissions.",
        key_difference: "Rust/400 user profiles are emulator identities and do not claim host authentication.",
        linux_example: "id && whoami",
        behavior_label: "Intentionally different",
        try_next: &["DSPUSRPRF", "DSPJOB"],
    },
    LinuxMapping {
        term: "FILESYSTEM",
        rust400_concept: "Libraries and the IFS analogy",
        rust400_command: "WRKLIB",
        linux_analogy: "A filesystem hierarchy with directories.",
        shared_idea: "Both organize named resources.",
        key_difference: "A Rust/400 library is a managed catalog concept, not just a host directory.",
        linux_example: "find . -maxdepth 2 -type d",
        behavior_label: "Historically inspired",
        try_next: &["WRKLIB", "DSPLIB", "CRTLIB"],
    },
    LinuxMapping {
        term: "PRINTSPOOL",
        rust400_concept: "Spooled files",
        rust400_command: "WRKSPLF",
        linux_analogy: "A print spool or queued text report.",
        shared_idea: "Output can be retained and viewed later.",
        key_difference: "Rust/400 spooled files are managed emulator records rather than arbitrary redirected output files.",
        linux_example: "lpq",
        behavior_label: "Planned",
        try_next: &["WRKSPLF", "DSPSPLF"],
    },
    LinuxMapping {
        term: "MESSAGEQUEUE",
        rust400_concept: "Operational messages",
        rust400_command: "DSPMSG",
        linux_analogy: "A mailbox, notification queue, or log stream.",
        shared_idea: "Information can be sent now and inspected later.",
        key_difference: "Rust/400 messages are tied to emulator workflows rather than one Linux facility such as syslog or mail.",
        linux_example: "journalctl -n 20",
        behavior_label: "Planned",
        try_next: &["SNDMSG", "DSPMSG"],
    },
];

pub fn find_mapping(term: &str) -> Option<&'static LinuxMapping> {
    MAPPINGS.iter().find(|mapping| mapping.term == term)
}

pub fn find_mapping_for_menu(menu_id: &str) -> Option<&'static LinuxMapping> {
    match menu_id {
        "LNXPATH" => find_mapping("PATH"),
        "LNXPROC" => find_mapping("PROCESS"),
        "LNXUSER" => find_mapping("USER"),
        "LNXFS" => find_mapping("FILESYSTEM"),
        "LNXSPL" => find_mapping("PRINTSPOOL"),
        "LNXMSG" => find_mapping("MESSAGEQUEUE"),
        _ => None,
    }
}

pub fn render_mapping(mapping: &LinuxMapping) -> String {
    format!(
        "Linux term: {}\nRust/400 concept: {}\nRust/400 command: {}\nLinux analogy: {}\nShared idea: {}\nKey difference: {}\nComparable Linux example:\n  {}\nBehavior label: {}\nTry next: {}",
        mapping.term,
        mapping.rust400_concept,
        mapping.rust400_command,
        mapping.linux_analogy,
        mapping.shared_idea,
        mapping.key_difference,
        mapping.linux_example,
        mapping.behavior_label,
        mapping.try_next.join(", ")
    )
}

#[cfg(test)]
mod tests {
    use super::{find_mapping, render_mapping};

    #[test]
    fn path_mapping_exists() {
        let mapping = find_mapping("PATH").expect("PATH mapping should exist");
        assert_eq!(mapping.rust400_command, "DSPLIBL");
    }

    #[test]
    fn rendered_mapping_contains_required_learning_fields() {
        let mapping = find_mapping("USER").expect("USER mapping should exist");
        let rendered = render_mapping(mapping);

        assert!(rendered.contains("Linux term: USER"));
        assert!(rendered.contains("Rust/400 concept: User profile"));
        assert!(rendered.contains("Comparable Linux example:"));
        assert!(rendered.contains("Behavior label:"));
    }
}
