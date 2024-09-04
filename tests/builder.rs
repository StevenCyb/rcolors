#[cfg(test)]
mod builder_tests {
    use rcolors::builder::Builder;

    #[test]
    fn builder_to_string() {
        let mut builder = Builder::new();
        builder.bg_black().text("text").reset().force_color();

        assert_eq!(builder.as_string(), "\x1b[40mtext\x1b[0m");
    }

    #[test]
    fn builder_print() {
        Builder::new()
            .bg_black()
            .text("text")
            .reset()
            .force_color()
            .print();
    }

    #[test]
    fn builder_thumbnail() {
        Builder::new()
            .force_color()
            // Color: an ANSI color package for Rust
            .fg_green()
            .text("Color: ")
            .fg_blue()
            .text("an ")
            .fg_white()
            .text("ANSI ")
            .fg_yellow()
            .text("color ")
            .fg_red()
            .text("package ")
            .fg_magenta()
            .text("for ")
            .fg_cyan()
            .text("Rust\n")
            // red text variations
            .fg_red()
            .text("red\t")
            .bg_red()
            .text("red    \t")
            .bg_hi_red()
            .text("red    \t")
            .reset()
            .fg_hi_red()
            .text(" red\n")
            .fg_green()
            .text("green\t")
            .bg_green()
            .text("green  \t")
            .bg_hi_green()
            .text("green  \t")
            .reset()
            .fg_hi_green()
            .text(" green\n")
            .fg_yellow()
            .text("yellow \t")
            .bg_yellow()
            .text("yellow \t")
            .bg_hi_yellow()
            .text("yellow \t")
            .reset()
            .fg_hi_yellow()
            .text(" yellow\n")
            .fg_blue()
            .text("blue   \t")
            .bg_blue()
            .text("blue   \t")
            .bg_hi_blue()
            .text("blue   \t")
            .reset()
            .fg_hi_blue()
            .text(" blue\n")
            .fg_magenta()
            .text("magenta\t")
            .bg_magenta()
            .text("magenta\t")
            .bg_hi_magenta()
            .text("magenta\t")
            .reset()
            .fg_hi_magenta()
            .text(" magenta\n")
            .fg_cyan()
            .text("cyan   \t")
            .bg_cyan()
            .text("cyan   \t")
            .bg_hi_cyan()
            .text("cyan   \t")
            .reset()
            .fg_hi_cyan()
            .text(" cyan\n")
            .fg_white()
            .text("white  \t")
            .bg_white()
            .text("white  \t")
            .bg_hi_white()
            .text("white  \t")
            .reset()
            .fg_hi_white()
            .text(" white\n")
            .print();
    }
}
