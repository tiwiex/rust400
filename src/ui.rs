use std::fmt::Write as _;

pub const INPUT_PROMPT: &str = "===> ";

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MainMenuScreen<'a> {
    pub system_name: &'a str,
    pub current_user: &'a str,
    pub job_name: &'a str,
    pub menu_id: &'a str,
}

pub fn render_main_menu(screen: &MainMenuScreen<'_>) -> String {
    const WIDTH: usize = 78;

    let mut output = String::new();
    let title = "IBM i Main Menu";
    let system_line = format!("System: {}", screen.system_name);

    writeln!(&mut output, "{}", " ".repeat(WIDTH)).expect("write should succeed");
    writeln!(
        &mut output,
        "{}{}",
        pad_right(screen.menu_id, 12),
        center_with_right(title, &system_line, WIDTH - 12)
    )
    .expect("write should succeed");
    writeln!(&mut output).expect("write should succeed");
    writeln!(&mut output, "  Select one of the following:").expect("write should succeed");
    writeln!(&mut output).expect("write should succeed");

    for option in menu_options() {
        writeln!(&mut output, "     {option}").expect("write should succeed");
    }

    writeln!(&mut output).expect("write should succeed");
    writeln!(&mut output, "  Selection or command").expect("write should succeed");
    writeln!(&mut output, "  {INPUT_PROMPT}").expect("write should succeed");
    writeln!(&mut output, "  {}", "-".repeat(WIDTH.saturating_sub(2)))
        .expect("write should succeed");
    writeln!(
        &mut output,
        "  F3=Exit   F4=Prompt   F9=Retrieve   F12=Cancel   F13=Information Assistant"
    )
    .expect("write should succeed");
    writeln!(&mut output, "  F23=Set initial menu").expect("write should succeed");
    writeln!(&mut output).expect("write should succeed");
    writeln!(
        &mut output,
        "  User: {:<12} Job: {}",
        screen.current_user, screen.job_name
    )
    .expect("write should succeed");

    output
}

fn menu_options() -> &'static [&'static str] {
    &[
        "1. User tasks",
        "2. Office tasks",
        "3. General system tasks",
        "4. Files, libraries, and folders",
        "5. Programming",
        "6. Communications",
        "7. Define or change the system",
        "8. Problem handling",
        "9. Display a menu",
        "10. Information Assistant options",
        "11. IBM i Access tasks",
        "",
        "90. Sign off",
    ]
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

#[cfg(test)]
mod tests {
    use super::{INPUT_PROMPT, MainMenuScreen, render_main_menu};

    #[test]
    fn main_menu_contains_expected_layout_regions() {
        let screen = MainMenuScreen {
            system_name: "NCRHEDEV",
            current_user: "MW",
            job_name: "QPADEV0001",
            menu_id: "MAIN",
        };

        let rendered = render_main_menu(&screen);

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
        let screen = MainMenuScreen {
            system_name: "NCRHEDEV",
            current_user: "MW",
            job_name: "QPADEV0001",
            menu_id: "MAIN",
        };

        let rendered = render_main_menu(&screen);

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
            "     ",
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
