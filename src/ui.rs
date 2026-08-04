use std::fmt::Write as _;

use crate::menus::MenuDefinition;

pub const INPUT_PROMPT: &str = "===> ";
const PANEL_WIDTH: usize = 78;
const VIEWPORT_WIDTH: usize = 110;
const TOP_PADDING_LINES: usize = 2;
const GREEN_SCREEN_PREFIX: &str = "\x1b[2J\x1b[H\x1b[40m\x1b[92m";
const GREEN_SCREEN_RESET: &str = "\x1b[0m";

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MenuScreen<'a> {
    pub menu: &'a MenuDefinition,
    pub system_name: &'a str,
    pub current_user: &'a str,
    pub job_name: &'a str,
}

pub fn render_menu(screen: &MenuScreen<'_>) -> String {
    let mut output = String::new();
    let system_line = format!("{}: {}", screen.menu.system_label, screen.system_name);
    let left_pad = " ".repeat((VIEWPORT_WIDTH - PANEL_WIDTH) / 2);
    let pad = |line: String| format!("{left_pad}{line}");

    for _ in 0..TOP_PADDING_LINES {
        writeln!(&mut output).expect("write should succeed");
    }

    writeln!(&mut output, "{}", pad(" ".repeat(PANEL_WIDTH))).expect("write should succeed");
    writeln!(
        &mut output,
        "{}{}",
        left_pad,
        format_args!(
            "{}{}",
            pad_right(screen.menu.id, 12),
            center_with_right(screen.menu.title, &system_line, PANEL_WIDTH - 12)
        )
    )
    .expect("write should succeed");
    writeln!(&mut output).expect("write should succeed");
    writeln!(
        &mut output,
        "{}",
        pad("  Select one of the following:".to_string())
    )
    .expect("write should succeed");
    writeln!(&mut output).expect("write should succeed");

    for option in screen.menu.options {
        writeln!(
            &mut output,
            "{}",
            pad(format!("     {}. {}", option.selector, option.label))
        )
        .expect("write should succeed");
    }

    writeln!(&mut output).expect("write should succeed");
    writeln!(
        &mut output,
        "{}",
        pad(format!("  {}", screen.menu.prompt_label))
    )
    .expect("write should succeed");
    writeln!(&mut output, "{}", pad(format!("  {INPUT_PROMPT}"))).expect("write should succeed");
    writeln!(
        &mut output,
        "{}",
        pad(format!("  {}", "-".repeat(PANEL_WIDTH.saturating_sub(2))))
    )
    .expect("write should succeed");

    for line in render_footer_hints(screen.menu.footer_hints).lines() {
        writeln!(&mut output, "{}", pad(format!("  {line}"))).expect("write should succeed");
    }

    writeln!(&mut output).expect("write should succeed");
    writeln!(
        &mut output,
        "{}",
        pad(format!(
            "  User: {:<12} Job: {}",
            screen.current_user, screen.job_name
        ))
    )
    .expect("write should succeed");

    output
}

pub fn render_green_screen(screen: &MenuScreen<'_>) -> String {
    format!(
        "{GREEN_SCREEN_PREFIX}{}{GREEN_SCREEN_RESET}",
        render_menu(screen)
    )
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
        format!("{}\n{}", first_row.join("   "), second_row.join("   "))
    }
}

#[cfg(test)]
mod tests {
    use crate::menus::find_menu;

    use super::{INPUT_PROMPT, MenuScreen, render_green_screen, render_menu};

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

        let expected = vec![
            "".to_string(),
            "".to_string(),
            "                                                                                                                                                          "
                .chars()
                .take(94)
                .collect(),
            "                MAIN                         IBM i Main Menu                  System: NCRHEDEV"
                .to_string(),
            "".to_string(),
            "                  Select one of the following:".to_string(),
            "".to_string(),
            "                     1. User tasks".to_string(),
            "                     2. Office tasks".to_string(),
            "                     3. General system tasks".to_string(),
            "                     4. Files, libraries, and folders".to_string(),
            "                     5. Programming".to_string(),
            "                     6. Communications".to_string(),
            "                     7. Define or change the system".to_string(),
            "                     8. Problem handling".to_string(),
            "                     9. Display a menu".to_string(),
            "                     10. Information Assistant options".to_string(),
            "                     11. IBM i Access tasks".to_string(),
            "                     90. Sign off".to_string(),
            "".to_string(),
            "                  Selection or command".to_string(),
            "                  ===> ".to_string(),
            "                  ----------------------------------------------------------------------------".to_string(),
            "                  F3=Exit   F4=Prompt   F9=Retrieve   F12=Cancel   F13=Information Assistant"
                .to_string(),
            "                  F23=Set initial menu".to_string(),
            "".to_string(),
            "                  User: MW           Job: QPADEV0001".to_string(),
            "".to_string(),
        ]
        .join("\n");

        assert_eq!(rendered, expected);
    }

    #[test]
    fn green_screen_wrapper_adds_terminal_styling() {
        let screen = MenuScreen {
            menu: find_menu("MAIN").expect("MAIN menu should exist"),
            system_name: "NCRHEDEV",
            current_user: "MW",
            job_name: "QPADEV0001",
        };

        let rendered = render_green_screen(&screen);

        assert!(rendered.starts_with("\x1b[2J\x1b[H\x1b[40m\x1b[92m"));
        assert!(rendered.ends_with("\x1b[0m"));
    }
}
