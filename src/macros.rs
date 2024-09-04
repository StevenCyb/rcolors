/// Macros for printing colored text.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// color_print!(Ansi::FgRed, "This is red text");
/// ``````
#[macro_export]
macro_rules! color_print {
    ($color:expr, $text:expr) => {{
        use $crate::ansi::Ansi;
        print!("{}{}{}", $color, $text, Ansi::Reset);
    }};
}

/// Print colored text with a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// color_println!(Ansi::FgRed, "This is red text");
/// ```
#[macro_export]
macro_rules! color_println {
    ($color:expr, $text:expr) => {{
        use $crate::ansi::Ansi;
        println!("{}{}{}", $color, $text, Ansi::Reset);
    }};
}

/// Return colored text as a string.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// let s = color_sprint!(Ansi::FgRed, "This is red text");
/// assert_eq!(s, "\u{1b}[31mThis is red text\u{1b}[0m");
/// ```
#[macro_export]
macro_rules! color_sprint {
    ($color:expr, $text:expr) => {{
        use $crate::ansi::Ansi;
        format!("{}{}{}", $color, $text, Ansi::Reset)
    }};
}

/// Print black text without a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// print_black!("This is black text");
/// ```
#[macro_export]
macro_rules! print_black {
    ($text:expr) => {{
        color_print!(Ansi::FgBlack, $text);
    }};
}

/// Print black text with a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// println_black!("This is black text");
/// ```
#[macro_export]
macro_rules! println_black {
    ($text:expr) => {{
        color_println!(Ansi::FgBlack, $text);
    }};
}

/// Return black text as a string.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// let s = black!("This is black text");
/// assert_eq!(s, "\u{1b}[30mThis is black text\u{1b}[0m");
/// ```
#[macro_export]
macro_rules! black {
    ($text:expr) => {{
        let s = color_sprint!(Ansi::FgBlack, $text);
        s
    }};
}

/// Print red text without a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// print_red!("This is red text");
/// ```
#[macro_export]
macro_rules! print_red {
    ($text:expr) => {{
        color_print!(Ansi::FgRed, $text);
    }};
}

/// Print red text with a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// println_red!("This is red text");
/// ```
#[macro_export]
macro_rules! println_red {
    ($text:expr) => {{
        color_println!(Ansi::FgRed, $text);
    }};
}

/// Return red text as a string.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// let s = red!("This is red text");
/// assert_eq!(s, "\u{1b}[31mThis is red text\u{1b}[0m");
/// ```
#[macro_export]
macro_rules! red {
    ($text:expr) => {{
        let s = color_sprint!(Ansi::FgRed, $text);
        s
    }};
}

/// Print green text to the console.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// print_green!("This is green text");
/// // This will print "\u{1b}[32mThis is green text\u{1b}[0m" to the console.
/// ```
#[macro_export]
macro_rules! print_green {
    ($text:expr) => {{
        color_print!(Ansi::FgGreen, $text);
    }};
}

/// Print green text to the console with a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// println_green!("This is green text");
/// // This will print "\u{1b}[32mThis is green text\u{1b}[0m\n" to the console.
/// ```
#[macro_export]
macro_rules! println_green {
    ($text:expr) => {{
        color_println!(Ansi::FgGreen, $text);
    }};
}

/// Return green text as a string.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// let s = green!("This is green text");
/// assert_eq!(s, "\u{1b}[32mThis is green text\u{1b}[0m");
/// ```
#[macro_export]
macro_rules! green {
    ($text:expr) => {{
        let s = color_sprint!(Ansi::FgGreen, $text);
        s
    }};
}

/// Print yellow text to the console.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// print_yellow!("This is yellow text");
/// // This will print "\u{1b}[33mThis is yellow text\u{1b}[0m" to the console.
/// ```
#[macro_export]
macro_rules! print_yellow {
    ($text:expr) => {{
        color_print!(Ansi::FgYellow, $text);
    }};
}

/// Print yellow text to the console with a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// println_yellow!("This is yellow text");
/// // This will print "\u{1b}[33mThis is yellow text\u{1b}[0m\n" to the console.
/// ```
#[macro_export]
macro_rules! println_yellow {
    ($text:expr) => {{
        color_println!(Ansi::FgYellow, $text);
    }};
}

/// Return yellow text as a string.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// let s = yellow!("This is yellow text");
/// assert_eq!(s, "\u{1b}[33mThis is yellow text\u{1b}[0m");
/// ```
#[macro_export]
macro_rules! yellow {
    ($text:expr) => {{
        let s = color_sprint!(Ansi::FgYellow, $text);
        s
    }};
}

/// Print blue text to the console.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// print_blue!("This is blue text");
/// // This will print "\u{1b}[34mThis is blue text\u{1b}[0m" to the console.
/// ```
#[macro_export]
macro_rules! print_blue {
    ($text:expr) => {{
        color_print!(Ansi::FgBlue, $text);
    }};
}

/// Print blue text to the console with a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// println_blue!("This is blue text");
/// // This will print "\u{1b}[34mThis is blue text\u{1b}[0m\n" to the console.
/// ```
#[macro_export]
macro_rules! println_blue {
    ($text:expr) => {{
        color_println!(Ansi::FgBlue, $text);
    }};
}

/// Return blue text as a string.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// let s = blue!("This is blue text");
/// assert_eq!(s, "\u{1b}[34mThis is blue text\u{1b}[0m");
/// ```
#[macro_export]
macro_rules! blue {
    ($text:expr) => {{
        let s = color_sprint!(Ansi::FgBlue, $text);
        s
    }};
}

/// Print magenta text to the console.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// print_magenta!("This is magenta text");
/// // This will print "\u{1b}[35mThis is magenta text\u{1b}[0m" to the console.
/// ```
#[macro_export]
macro_rules! print_magenta {
    ($text:expr) => {{
        color_print!(Ansi::FgMagenta, $text);
    }};
}

/// Print magenta text to the console with a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// println_magenta!("This is magenta text");
/// // This will print "\u{1b}[35mThis is magenta text\u{1b}[0m\n" to the console.
/// ```
#[macro_export]
macro_rules! println_magenta {
    ($text:expr) => {{
        color_println!(Ansi::FgMagenta, $text);
    }};
}

/// Return magenta text as a string.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// let s = magenta!("This is magenta text");
/// assert_eq!(s, "\u{1b}[35mThis is magenta text\u{1b}[0m");
/// ```
#[macro_export]
macro_rules! magenta {
    ($text:expr) => {{
        let s = color_sprint!(Ansi::FgMagenta, $text);
        s
    }};
}

/// Print cyan text to the console.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// print_cyan!("This is cyan text");
/// // This will print "\u{1b}[36mThis is cyan text\u{1b}[0m" to the console.
/// ```
#[macro_export]
macro_rules! print_cyan {
    ($text:expr) => {{
        color_print!(Ansi::FgCyan, $text);
    }};
}

/// Print cyan text to the console with a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// println_cyan!("This is cyan text");
/// // This will print "\u{1b}[36mThis is cyan text\u{1b}[0m\n" to the console.
/// ```
#[macro_export]
macro_rules! println_cyan {
    ($text:expr) => {{
        color_println!(Ansi::FgCyan, $text);
    }};
}

/// Return cyan text as a string.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// let s = cyan!("This is cyan text");
/// assert_eq!(s, "\u{1b}[36mThis is cyan text\u{1b}[0m");
/// ```
#[macro_export]
macro_rules! cyan {
    ($text:expr) => {{
        let s = color_sprint!(Ansi::FgCyan, $text);
        s
    }};
}

/// Print white text to the console.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// print_white!("This is white text");
/// // This will print "\u{1b}[37mThis is white text\u{1b}[0m" to the console.
/// ```
#[macro_export]
macro_rules! print_white {
    ($text:expr) => {{
        color_print!(Ansi::FgWhite, $text);
    }};
}

/// Print white text to the console with a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// println_white!("This is white text");
/// // This will print "\u{1b}[37mThis is white text\u{1b}[0m\n" to the console.
/// ```
#[macro_export]
macro_rules! println_white {
    ($text:expr) => {{
        color_println!(Ansi::FgWhite, $text);
    }};
}

/// Return white text as a string.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// let s = white!("This is white text");
/// assert_eq!(s, "\u{1b}[37mThis is white text\u{1b}[0m");
/// ```
#[macro_export]
macro_rules! white {
    ($text:expr) => {{
        let s = color_sprint!(Ansi::FgWhite, $text);
        s
    }};
}

/// Print high-intensity black text to the console.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// print_hi_black!("This is hi-black text");
/// // This will print "\u{1b}[90mThis is hi-black text\u{1b}[0m" to the console.
/// ```
#[macro_export]
macro_rules! print_hi_black {
    ($text:expr) => {{
        color_print!(Ansi::FgHiBlack, $text);
    }};
}

/// Print high-intensity black text to the console with a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// println_hi_black!("This is hi-black text");
/// // This will print "\u{1b}[90mThis is hi-black text\u{1b}[0m\n" to the console.
/// ```
#[macro_export]
macro_rules! println_hi_black {
    ($text:expr) => {{
        color_println!(Ansi::FgHiBlack, $text);
    }};
}

/// Return high-intensity black text as a string.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// let s = hi_black!("This is hi-black text");
/// assert_eq!(s, "\u{1b}[90mThis is hi-black text\u{1b}[0m");
/// ```
#[macro_export]
macro_rules! hi_black {
    ($text:expr) => {{
        let s = color_sprint!(Ansi::FgHiBlack, $text);
        s
    }};
}

/// Print high-intensity red text to the console.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// print_hi_red!("This is hi-red text");
/// // This will print "\u{1b}[91mThis is hi-red text\u{1b}[0m" to the console.
/// ```
#[macro_export]
macro_rules! print_hi_red {
    ($text:expr) => {{
        color_print!(Ansi::FgHiRed, $text);
    }};
}

/// Print high-intensity red text to the console with a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// println_hi_red!("This is hi-red text");
/// // This will print "\u{1b}[91mThis is hi-red text\u{1b}[0m\n" to the console.
/// ```
#[macro_export]
macro_rules! println_hi_red {
    ($text:expr) => {{
        color_println!(Ansi::FgHiRed, $text);
    }};
}

/// Return high-intensity red text as a string.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// let s = hi_red!("This is hi-red text");
/// assert_eq!(s, "\u{1b}[91mThis is hi-red text\u{1b}[0m");
/// ```
#[macro_export]
macro_rules! hi_red {
    ($text:expr) => {{
        let s = color_sprint!(Ansi::FgHiRed, $text);
        s
    }};
}

/// Print high-intensity green text to the console.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// print_hi_green!("This is hi-green text");
/// // This will print "\u{1b}[92mThis is hi-green text\u{1b}[0m" to the console.
/// ```
#[macro_export]
macro_rules! print_hi_green {
    ($text:expr) => {{
        color_print!(Ansi::FgHiGreen, $text);
    }};
}

/// Print high-intensity green text to the console with a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// println_hi_green!("This is hi-green text");
/// // This will print "\u{1b}[92mThis is hi-green text\u{1b}[0m\n" to the console.
/// ```
#[macro_export]
macro_rules! println_hi_green {
    ($text:expr) => {{
        color_println!(Ansi::FgHiGreen, $text);
    }};
}

/// Return high-intensity green text as a string.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// let s = hi_green!("This is hi-green text");
/// assert_eq!(s, "\u{1b}[92mThis is hi-green text\u{1b}[0m");
/// ```
#[macro_export]
macro_rules! hi_green {
    ($text:expr) => {{
        let s = color_sprint!(Ansi::FgHiGreen, $text);
        s
    }};
}

/// Print high-intensity yellow text to the console.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// print_hi_yellow!("This is hi-yellow text");
/// // This will print "\u{1b}[93mThis is hi-yellow text\u{1b}[0m" to the console.
/// ```
#[macro_export]
macro_rules! print_hi_yellow {
    ($text:expr) => {{
        color_print!(Ansi::FgHiYellow, $text);
    }};
}

/// Print high-intensity yellow text to the console with a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// println_hi_yellow!("This is hi-yellow text");
/// // This will print "\u{1b}[93mThis is hi-yellow text\u{1b}[0m\n" to the console.
/// ```
#[macro_export]
macro_rules! println_hi_yellow {
    ($text:expr) => {{
        color_println!(Ansi::FgHiYellow, $text);
    }};
}

/// Return high-intensity yellow text as a string.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// let s = hi_yellow!("This is hi-yellow text");
/// assert_eq!(s, "\u{1b}[93mThis is hi-yellow text\u{1b}[0m");
/// ```
#[macro_export]
macro_rules! hi_yellow {
    ($text:expr) => {{
        let s = color_sprint!(Ansi::FgHiYellow, $text);
        s
    }};
}

/// Print high-intensity blue text to the console.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// print_hi_blue!("This is hi-blue text");
/// // This will print "\u{1b}[94mThis is hi-blue text\u{1b}[0m" to the console.
/// ```
#[macro_export]
macro_rules! print_hi_blue {
    ($text:expr) => {{
        color_print!(Ansi::FgHiBlue, $text);
    }};
}

/// Print high-intensity blue text to the console with a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// println_hi_blue!("This is hi-blue text");
/// // This will print "\u{1b}[94mThis is hi-blue text\u{1b}[0m\n" to the console.
/// ```
#[macro_export]
macro_rules! println_hi_blue {
    ($text:expr) => {{
        color_println!(Ansi::FgHiBlue, $text);
    }};
}

/// Return high-intensity blue text as a string.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// let s = hi_blue!("This is hi-blue text");
/// assert_eq!(s, "\u{1b}[94mThis is hi-blue text\u{1b}[0m");
/// ```
#[macro_export]
macro_rules! hi_blue {
    ($text:expr) => {{
        let s = color_sprint!(Ansi::FgHiBlue, $text);
        s
    }};
}

/// Print high-intensity magenta text to the console.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// print_hi_magenta!("This is hi-magenta text");
/// // This will print "\u{1b}[95mThis is hi-magenta text\u{1b}[0m" to the console.
/// ```
#[macro_export]
macro_rules! print_hi_magenta {
    ($text:expr) => {{
        color_print!(Ansi::FgHiMagenta, $text);
    }};
}

/// Print high-intensity magenta text to the console with a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// println_hi_magenta!("This is hi-magenta text");
/// // This will print "\u{1b}[95mThis is hi-magenta text\u{1b}[0m\n" to the console.
/// ```
#[macro_export]
macro_rules! println_hi_magenta {
    ($text:expr) => {{
        color_println!(Ansi::FgHiMagenta, $text);
    }};
}

/// Return high-intensity magenta text as a string.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// let s = hi_magenta!("This is hi-magenta text");
/// assert_eq!(s, "\u{1b}[95mThis is hi-magenta text\u{1b}[0m");
/// ```
#[macro_export]
macro_rules! hi_magenta {
    ($text:expr) => {{
        let s = color_sprint!(Ansi::FgHiMagenta, $text);
        s
    }};
}

/// Print high-intensity cyan text to the console.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// print_hi_cyan!("This is hi-cyan text");
/// // This will print "\u{1b}[96mThis is hi-cyan text\u{1b}[0m" to the console.
/// ```
#[macro_export]
macro_rules! print_hi_cyan {
    ($text:expr) => {{
        color_print!(Ansi::FgHiCyan, $text);
    }};
}

/// Print high-intensity cyan text to the console with a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// println_hi_cyan!("This is hi-cyan text");
/// // This will print "\u{1b}[96mThis is hi-cyan text\u{1b}[0m\n" to the console.
/// ```
#[macro_export]
macro_rules! println_hi_cyan {
    ($text:expr) => {{
        color_println!(Ansi::FgHiCyan, $text);
    }};
}

/// Return high-intensity cyan text as a string.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// let s = hi_cyan!("This is hi-cyan text");
/// assert_eq!(s, "\u{1b}[96mThis is hi-cyan text\u{1b}[0m");
/// ```
#[macro_export]
macro_rules! hi_cyan {
    ($text:expr) => {{
        let s = color_sprint!(Ansi::FgHiCyan, $text);
        s
    }};
}

/// Print high-intensity white text to the console.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// print_hi_white!("This is hi-white text");
/// // This will print "\u{1b}[97mThis is hi-white text\u{1b}[0m" to the console.
/// ```
#[macro_export]
macro_rules! print_hi_white {
    ($text:expr) => {{
        color_print!(Ansi::FgHiWhite, $text);
    }};
}

/// Print high-intensity white text to the console with a newline.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// println_hi_white!("This is hi-white text");
/// // This will print "\u{1b}[97mThis is hi-white text\u{1b}[0m\n" to the console.
/// ```
#[macro_export]
macro_rules! println_hi_white {
    ($text:expr) => {{
        color_println!(Ansi::FgHiWhite, $text);
    }};
}

/// Return high-intensity white text as a string.
///
/// # Example
///
/// ```
/// use rcolors::*;
///
/// let s = hi_white!("This is hi-white text");
/// assert_eq!(s, "\u{1b}[97mThis is hi-white text\u{1b}[0m");
/// ```
#[macro_export]
macro_rules! hi_white {
    ($text:expr) => {{
        let s = color_sprint!(Ansi::FgHiWhite, $text);
        s
    }};
}

#[cfg(test)]
mod macro_tests {
    #[test]
    fn macro_color_print() {
        color_print!(Ansi::FgRed, "This is red text");
    }

    #[test]
    fn macro_color_println() {
        color_println!(Ansi::FgRed, "This is red text");
    }

    #[test]
    fn macro_color_sprint() {
        let s = color_sprint!(Ansi::FgRed, "This is red text");
        assert_eq!(s, "\u{1b}[31mThis is red text\u{1b}[0m");
    }
}
