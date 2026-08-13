#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MenuAction {
    OpenMenu(&'static str),
    RunCommand(&'static str),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FunctionKeyAction {
    Exit,
    Prompt,
    Retrieve,
    Cancel,
    Help,
    SetInitialMenu,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FooterHint {
    pub key: &'static str,
    pub label: &'static str,
    pub action: FunctionKeyAction,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MenuOption {
    pub selector: &'static str,
    pub label: &'static str,
    pub action: MenuAction,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MenuDefinition {
    pub id: &'static str,
    pub title: &'static str,
    pub prompt_label: &'static str,
    pub system_label: &'static str,
    pub options: &'static [MenuOption],
    pub footer_hints: &'static [FooterHint],
}

const MAIN_MENU_OPTIONS: &[MenuOption] = &[
    MenuOption {
        selector: "1",
        label: "User tasks",
        action: MenuAction::OpenMenu("USR"),
    },
    MenuOption {
        selector: "2",
        label: "Office tasks",
        action: MenuAction::OpenMenu("OFFICE"),
    },
    MenuOption {
        selector: "3",
        label: "General system tasks",
        action: MenuAction::OpenMenu("GENSYS"),
    },
    MenuOption {
        selector: "4",
        label: "Files, libraries, and folders",
        action: MenuAction::OpenMenu("FILES"),
    },
    MenuOption {
        selector: "5",
        label: "Programming",
        action: MenuAction::OpenMenu("PGM"),
    },
    MenuOption {
        selector: "6",
        label: "Communications",
        action: MenuAction::OpenMenu("COMM"),
    },
    MenuOption {
        selector: "7",
        label: "Define or change the system",
        action: MenuAction::OpenMenu("CFG"),
    },
    MenuOption {
        selector: "8",
        label: "Problem handling",
        action: MenuAction::OpenMenu("PROBLEM"),
    },
    MenuOption {
        selector: "9",
        label: "Display a menu",
        action: MenuAction::RunCommand("WRKCMD"),
    },
    MenuOption {
        selector: "10",
        label: "Information Assistant options",
        action: MenuAction::OpenMenu("INFO"),
    },
    MenuOption {
        selector: "11",
        label: "IBM i Access tasks",
        action: MenuAction::OpenMenu("ACCESS"),
    },
    MenuOption {
        selector: "12",
        label: "Linux mappings",
        action: MenuAction::OpenMenu("LNX"),
    },
    MenuOption {
        selector: "90",
        label: "Sign off",
        action: MenuAction::RunCommand("EXIT"),
    },
];

const MAIN_MENU_HINTS: &[FooterHint] = &[
    FooterHint {
        key: "F3",
        label: "Exit",
        action: FunctionKeyAction::Exit,
    },
    FooterHint {
        key: "F4",
        label: "Prompt",
        action: FunctionKeyAction::Prompt,
    },
    FooterHint {
        key: "F9",
        label: "Retrieve",
        action: FunctionKeyAction::Retrieve,
    },
    FooterHint {
        key: "F12",
        label: "Cancel",
        action: FunctionKeyAction::Cancel,
    },
    FooterHint {
        key: "F13",
        label: "Information Assistant",
        action: FunctionKeyAction::Help,
    },
    FooterHint {
        key: "F23",
        label: "Set initial menu",
        action: FunctionKeyAction::SetInitialMenu,
    },
];

const USER_MENU_OPTIONS: &[MenuOption] = &[
    MenuOption {
        selector: "1",
        label: "Display current user profile",
        action: MenuAction::RunCommand("DSPUSRPRF"),
    },
    MenuOption {
        selector: "2",
        label: "Display current job",
        action: MenuAction::RunCommand("DSPJOB"),
    },
    MenuOption {
        selector: "3",
        label: "Send a message",
        action: MenuAction::RunCommand("SNDMSG"),
    },
    MenuOption {
        selector: "4",
        label: "Display a library",
        action: MenuAction::RunCommand("DSPLIB"),
    },
    MenuOption {
        selector: "5",
        label: "Create a library",
        action: MenuAction::RunCommand("CRTLIB"),
    },
    MenuOption {
        selector: "90",
        label: "Return to main menu",
        action: MenuAction::OpenMenu("MAIN"),
    },
];

const USER_MENU_HINTS: &[FooterHint] = MAIN_MENU_HINTS;

const LINUX_MENU_OPTIONS: &[MenuOption] = &[
    MenuOption {
        selector: "1",
        label: "PATH and library lists",
        action: MenuAction::OpenMenu("LNXPATH"),
    },
    MenuOption {
        selector: "2",
        label: "Processes and jobs",
        action: MenuAction::OpenMenu("LNXPROC"),
    },
    MenuOption {
        selector: "3",
        label: "Linux users and user profiles",
        action: MenuAction::OpenMenu("LNXUSER"),
    },
    MenuOption {
        selector: "4",
        label: "Filesystem and libraries",
        action: MenuAction::OpenMenu("LNXFS"),
    },
    MenuOption {
        selector: "5",
        label: "Print spool and spooled files",
        action: MenuAction::OpenMenu("LNXSPL"),
    },
    MenuOption {
        selector: "6",
        label: "Message queue analogies",
        action: MenuAction::OpenMenu("LNXMSG"),
    },
    MenuOption {
        selector: "90",
        label: "Return to main menu",
        action: MenuAction::OpenMenu("MAIN"),
    },
];

const LINUX_MENU_HINTS: &[FooterHint] = MAIN_MENU_HINTS;

const LINUX_DETAIL_OPTIONS: &[MenuOption] = &[MenuOption {
    selector: "90",
    label: "Return to Linux mappings",
    action: MenuAction::OpenMenu("LNX"),
}];

const MENUS: &[MenuDefinition] = &[
    MenuDefinition {
        id: "MAIN",
        title: "IBM i Main Menu",
        prompt_label: "Selection or command",
        system_label: "System",
        options: MAIN_MENU_OPTIONS,
        footer_hints: MAIN_MENU_HINTS,
    },
    MenuDefinition {
        id: "USR",
        title: "User Tasks",
        prompt_label: "Selection or command",
        system_label: "System",
        options: USER_MENU_OPTIONS,
        footer_hints: USER_MENU_HINTS,
    },
    MenuDefinition {
        id: "LNX",
        title: "Linux Mappings",
        prompt_label: "Selection or command",
        system_label: "System",
        options: LINUX_MENU_OPTIONS,
        footer_hints: LINUX_MENU_HINTS,
    },
    MenuDefinition {
        id: "LNXPATH",
        title: "Linux Mapping Detail",
        prompt_label: "Selection or command",
        system_label: "System",
        options: LINUX_DETAIL_OPTIONS,
        footer_hints: LINUX_MENU_HINTS,
    },
    MenuDefinition {
        id: "LNXPROC",
        title: "Linux Mapping Detail",
        prompt_label: "Selection or command",
        system_label: "System",
        options: LINUX_DETAIL_OPTIONS,
        footer_hints: LINUX_MENU_HINTS,
    },
    MenuDefinition {
        id: "LNXUSER",
        title: "Linux Mapping Detail",
        prompt_label: "Selection or command",
        system_label: "System",
        options: LINUX_DETAIL_OPTIONS,
        footer_hints: LINUX_MENU_HINTS,
    },
    MenuDefinition {
        id: "LNXFS",
        title: "Linux Mapping Detail",
        prompt_label: "Selection or command",
        system_label: "System",
        options: LINUX_DETAIL_OPTIONS,
        footer_hints: LINUX_MENU_HINTS,
    },
    MenuDefinition {
        id: "LNXSPL",
        title: "Linux Mapping Detail",
        prompt_label: "Selection or command",
        system_label: "System",
        options: LINUX_DETAIL_OPTIONS,
        footer_hints: LINUX_MENU_HINTS,
    },
    MenuDefinition {
        id: "LNXMSG",
        title: "Linux Mapping Detail",
        prompt_label: "Selection or command",
        system_label: "System",
        options: LINUX_DETAIL_OPTIONS,
        footer_hints: LINUX_MENU_HINTS,
    },
];

pub fn registered_menus() -> &'static [MenuDefinition] {
    MENUS
}

pub fn find_menu(id: &str) -> Option<&'static MenuDefinition> {
    registered_menus().iter().find(|menu| menu.id == id)
}

pub fn find_option<'a>(menu: &'a MenuDefinition, selection: &str) -> Option<&'a MenuOption> {
    menu.options
        .iter()
        .find(|option| option.selector.eq_ignore_ascii_case(selection))
}

pub fn find_footer_hint<'a>(menu: &'a MenuDefinition, key: &str) -> Option<&'a FooterHint> {
    menu.footer_hints
        .iter()
        .find(|hint| hint.key.eq_ignore_ascii_case(key))
}

pub fn validate_menu_registry(menus: &[MenuDefinition]) -> Result<(), &'static str> {
    for menu in menus {
        if menu.id.trim().is_empty() {
            return Err("registered menu is missing an id");
        }

        if menu.title.trim().is_empty() {
            return Err("registered menu is missing a title");
        }

        if menu.prompt_label.trim().is_empty() {
            return Err("registered menu is missing a prompt label");
        }

        if menu.system_label.trim().is_empty() {
            return Err("registered menu is missing a system label");
        }

        if menu.options.is_empty() {
            return Err("registered menu is missing options");
        }

        for option in menu.options {
            if option.selector.trim().is_empty() {
                return Err("registered menu option is missing a selector");
            }

            if option.label.trim().is_empty() {
                return Err("registered menu option is missing a label");
            }
        }

        if menu.footer_hints.is_empty() {
            return Err("registered menu is missing footer hints");
        }

        for hint in menu.footer_hints {
            if hint.key.trim().is_empty() {
                return Err("registered footer hint is missing a key");
            }

            if hint.label.trim().is_empty() {
                return Err("registered footer hint is missing a label");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        FooterHint, FunctionKeyAction, MenuAction, MenuDefinition, MenuOption, find_footer_hint,
        find_menu, validate_menu_registry,
    };

    #[test]
    fn main_menu_is_registered_with_expected_metadata() {
        let menu = find_menu("MAIN").expect("MAIN menu should be registered");

        assert_eq!(menu.title, "IBM i Main Menu");
        assert_eq!(menu.prompt_label, "Selection or command");
        assert_eq!(menu.options[0].selector, "1");
        assert_eq!(menu.options[0].label, "User tasks");
        assert_eq!(menu.options[12].selector, "90");
        assert_eq!(menu.options[12].action, MenuAction::RunCommand("EXIT"));
        assert_eq!(
            find_footer_hint(menu, "F3")
                .expect("F3 should exist")
                .action,
            FunctionKeyAction::Exit
        );

        let user_menu = find_menu("USR").expect("USR menu should be registered");
        assert_eq!(user_menu.title, "User Tasks");
        assert_eq!(
            user_menu.options[0].action,
            MenuAction::RunCommand("DSPUSRPRF")
        );
        assert_eq!(
            user_menu.options[1].action,
            MenuAction::RunCommand("DSPJOB")
        );
        assert_eq!(user_menu.options[5].action, MenuAction::OpenMenu("MAIN"));

        let linux_menu = find_menu("LNX").expect("LNX menu should be registered");
        assert_eq!(linux_menu.title, "Linux Mappings");
        assert_eq!(
            linux_menu.options[0].action,
            MenuAction::OpenMenu("LNXPATH")
        );
        assert_eq!(linux_menu.options[6].action, MenuAction::OpenMenu("MAIN"));

        let linux_detail = find_menu("LNXPATH").expect("LNXPATH menu should be registered");
        assert_eq!(linux_detail.options[0].action, MenuAction::OpenMenu("LNX"));
    }

    #[test]
    fn metadata_validation_fails_when_a_menu_title_is_missing() {
        let invalid = [MenuDefinition {
            id: "BROKEN",
            title: "",
            prompt_label: "Selection or command",
            system_label: "System",
            options: &[MenuOption {
                selector: "1",
                label: "Anything",
                action: MenuAction::OpenMenu("X"),
            }],
            footer_hints: &[FooterHint {
                key: "F3",
                label: "Exit",
                action: FunctionKeyAction::Exit,
            }],
        }];

        assert_eq!(
            validate_menu_registry(&invalid),
            Err("registered menu is missing a title")
        );
    }
}
