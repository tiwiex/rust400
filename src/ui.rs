use std::fmt::Write as _;

use crate::menus::MenuDefinition;

pub const INPUT_PROMPT: &str = "===> ";

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MenuScreen<'a> {
    pub menu: &'a MenuDefinition,
    pub system_name: &'a str,
    pub current_user: &'a str,
    pub job_name: &'a str,
}

pub fn render_menu(screen: &MenuScreen<'_>) -> String {
    const WIDTH: usize = 78;

    let mut output = String::new();
    let system_line = format!("{}: {}", screen.menu.system_label, screen.system_name);

    writeln!(&mut output, "{}", " ".repeat(WIDTH)).expect("write should succeed");
    writeln!(
        &mut output,
        "{}{}",
        pad_right(screen.menu.id, 12),
        center_with_right(screen.menu.title, &system_line, WIDTH - 12)
    )
    .expect("write should succeed");
    writeln!(&mut output).expect("write should succeed");
    writeln!(&mut output, "  Select one of the following:").expect("write should succeed");
    writeln!(&mut output).expect("write should succeed");

    for option in screen.menu.options {
        writeln!(&mut output, "     {}. {}", option.selector, option.label)
            .expect("write should succeed");
    }

    writeln!(&mut output).expect("write should succeed");
    writeln!(&mut output, "  {}", screen.menu.prompt_label).expect("write should succeed");
    writeln!(&mut output, "  {INPUT_PROMPT}").expect("write should succeed");
    writeln!(&mut output, "  {}", "-".repeat(WIDTH.saturating_sub(2)))
        .expect("write should succeed");
    writeln!(
        &mut output,
        "  {}",
        render_footer_hints(screen.menu.footer_hints)
    )
    .expect("write should succeed");
    writeln!(&mut output).expect("write should succeed");
    writeln!(
        &mut output,
        "  User: {:<12} Job: {}",
        screen.current_user, screen.job_name
    )
    .expect("write should succeed");

    output
}

fn pad_right(text: &str, width: usize) -> String {
    format!("{text:<width$}")
}

fn center_with_right(title: &str, right_text: &str, width: usize) -> String {
    if right_text.len() >= width {
        return pad_right(right_text, width);
    }

    let left_width = width - right_text.len();
    let title_start = left_width.saturating_sub(title.len()) / 2;
    let mut chars = vec![' '; width];

    for (index, character) in title.chars().enumerate() {
        if title_start + index < left_width {
            chars[title_start + index] = character;
        }
    }

    let right_start = width - right_text.len();

    for (index, character) in right_text.chars().enumerate() {
        chars[right_start + index] = character;
    }

    chars.into_iter().collect()
}

fn render_footer_hints(hints: &[crate::menus::FooterHint]) -> String {
    let mut first_row = Vec::new();
    let mut second_row = Vec::new();

    for (index, hint) in hints.iter().enumerate() {
        let formatted = format!("{}={}", hint.key, hint.label);

        if index < 5 {
            first_row.push(formatted);
        } else {
            second_row.push(formatted);
        }
    }

    if second_row.is_empty() {
        first_row.join("   ")
    } else {
        format!("{}\n  {}", first_row.join("   "), second_row.join("   "))
    }
}

#[cfg(test)]
mod tests {
    use crate::menus::find_menu;

    use super::{INPUT_PROMPT, MenuScreen, render_menu};

    #[test]
    fn main_menu_contains_expected_layout_regions() {
        let screen = MenuScreen {
            menu: find_menu("MAIN").expect("MAIN menu should exist"),
            system_name: "NCRHEDEV",
            current_user: "MW",
            job_name: "QPADEV0001",
        };

        let rendered = render_menu(&screen);

        assert!(rendered.contains("MAIN"));
        assert!(rendered.contains("IBM i Main Menu"));
        assert!(rendered.contains("System: NCRHEDEV"));
        assert!(rendered.contains("Select one of the following:"));
        assert!(rendered.contains("1. User tasks"));
        assert!(rendered.contains("90. Sign off"));
        assert!(rendered.contains("Selection or command"));
        assert!(rendered.contains(INPUT_PROMPT));
        assert!(rendered.contains("F3=Exit"));
        assert!(rendered.contains("F12=Cancel"));
        assert!(rendered.contains("User: MW"));
        assert!(rendered.contains("Job: QPADEV0001"));
    }

    #[test]
    fn main_menu_matches_the_snapshot_layout() {
        let screen = MenuScreen {
            menu: find_menu("MAIN").expect("MAIN menu should exist"),
            system_name: "NCRHEDEV",
            current_user: "MW",
            job_name: "QPADEV0001",
        };

        let rendered = render_menu(&screen);

        let expected = [
            "                                                                              ",
            "MAIN                         IBM i Main Menu                  System: NCRHEDEV",
            "",
            "  Select one of the following:",
            "",
            "     1. User tasks",
            "     2. Office tasks",
            "     3. General system tasks",
            "     4. Files, libraries, and folders",
            "     5. Programming",
            "     6. Communications",
            "     7. Define or change the system",
            "     8. Problem handling",
            "     9. Display a menu",
            "     10. Information Assistant options",
            "     11. IBM i Access tasks",
            "     90. Sign off",
            "",
            "  Selection or command",
            "  ===> ",
            "  ----------------------------------------------------------------------------",
            "  F3=Exit   F4=Prompt   F9=Retrieve   F12=Cancel   F13=Information Assistant",
            "  F23=Set initial menu",
            "",
            "  User: MW           Job: QPADEV0001",
            "",
        ]
        .join("\n");

        assert_eq!(rendered, expected);
    }
}
