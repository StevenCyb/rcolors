use std::fmt::{self, Display};

/// Represents a terminal color attribute using ANSI escape codes.
///
/// # Examples
///
/// ```
/// use rcolors::ansi::Ansi;
///
/// let fg_green = Ansi::FgGreen;
/// assert_eq!(format!("{}", fg_green), "\x1b[32m");
///
/// let bg_red = Ansi::BgRed;
/// assert_eq!(format!("{}", bg_red), "\x1b[41m");
/// ```
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ansi {
    // Control,
    Reset = 0,
    Bold = 1,
    Faint = 2,
    Italic = 3,
    Underline = 4,
    BlinkSlow = 5,
    BlinkRapid = 6,
    ReverseVideo = 7,
    Concealed = 8,
    CrossedOut = 9,
    // FG
    FgBlack = 30,
    FgRed = 31,
    FgGreen = 32,
    FgYellow = 33,
    FgBlue = 34,
    FgMagenta = 35,
    FgCyan = 36,
    FgWhite = 37,
    FgHiBlack = 90,
    FgHiRed = 91,
    FgHiGreen = 92,
    FgHiYellow = 93,
    FgHiBlue = 94,
    FgHiMagenta = 95,
    FgHiCyan = 96,
    FgHiWhite = 97,
    // BG
    BgBlack = 40,
    BgRed = 41,
    BgGreen = 42,
    BgYellow = 43,
    BgBlue = 44,
    BgMagenta = 45,
    BgCyan = 46,
    BgWhite = 47,
    BgHiBlack = 100,
    BgHiRed = 101,
    BgHiGreen = 102,
    BgHiYellow = 103,
    BgHiBlue = 104,
    BgHiMagenta = 105,
    BgHiCyan = 106,
    BgHiWhite = 107,
}

// Implement the Display trait for the Color enum.
impl Display for Ansi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[{}m", *self as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::Ansi;

    #[test]
    fn test_fg_colors() {
        let fg_green = Ansi::FgGreen;
        assert_eq!(format!("{}", fg_green), "\x1b[32m");

        let fg_hi_red = Ansi::FgHiRed;
        assert_eq!(format!("{}", fg_hi_red), "\x1b[91m");
    }

    #[test]
    fn test_bg_colors() {
        let bg_black = Ansi::BgBlack;
        assert_eq!(format!("{}", bg_black), "\x1b[40m");

        let bg_hi_white = Ansi::BgHiWhite;
        assert_eq!(format!("{}", bg_hi_white), "\x1b[107m");
    }

    #[test]
    fn test_control_codes() {
        let reset = Ansi::Reset;
        assert_eq!(format!("{}", reset), "\x1b[0m");

        let bold = Ansi::Bold;
        assert_eq!(format!("{}", bold), "\x1b[1m");
    }
}
