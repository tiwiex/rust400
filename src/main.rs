const STARTUP_MESSAGE: &str =
    "Rust/400 development shell: initialization complete; no commands are available yet.";

fn main() {
    println!("{STARTUP_MESSAGE}");
}

#[cfg(test)]
mod tests {
    use super::STARTUP_MESSAGE;

    #[test]
    fn startup_message_identifies_the_placeholder_state() {
        assert!(STARTUP_MESSAGE.starts_with("Rust/400 development shell:"));
        assert!(STARTUP_MESSAGE.contains("no commands are available yet"));
    }
}
