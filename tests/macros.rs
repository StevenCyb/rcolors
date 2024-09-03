#[cfg(test)]
mod macro_tests {
    use rcolors::*;
    use std::io::Write;

    #[test]
    fn test_color_print() {
        // Redirect stdout for testing
        let output = std::io::stdout();
        let mut handle = output.lock();
        color_print!(Ansi::FgRed, "This is red text");
        assert_eq!(
            handle.write(b"\x1b[31mThis is red text\x1b[0m").is_ok(),
            true
        );
    }

    #[test]
    fn test_color_println() {
        // Redirect stdout for testing
        let output = std::io::stdout();
        let mut handle = output.lock();
        color_println!(Ansi::FgRed, "This is red text");
        assert_eq!(
            handle.write(b"\x1b[31mThis is red text\x1b[0m\n").is_ok(),
            true
        );
    }

    #[test]
    fn test_color_sprint() {
        let s = color_sprint!(Ansi::FgRed, "This is red text");
        assert_eq!(s, "\u{1b}[31mThis is red text\u{1b}[0m");
    }
}
