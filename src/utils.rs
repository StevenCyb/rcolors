pub fn no_color() -> bool {
    std::env::var("NO_COLOR").is_ok() || !atty::is(atty::Stream::Stdout)
}
