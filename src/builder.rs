use crate::ansi::Ansi;
use crate::utils;
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq)]
pub enum Entity {
    Text(String),
    Ansi(Ansi),
}

/// A builder for creating styled and colored text.
/// This builder can be used to create styled and colored text.
/// The builder can be printed to stdout or returned as a `String`.
/// The builder can also be used to create custom ANSI codes.
///
/// # Examples
///
/// ```
/// use rcolors::builder::Builder;
///
/// let mut builder = Builder::new();
/// builder.text("Hello, world!").bold().text(" This is bold text!").reset();
/// builder.print();
/// ```
#[derive(Debug, Clone)]
pub struct Builder {
    content: Vec<Entity>,
    no_color: bool,
    force_color: bool,
}

impl Display for Builder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())?;
        Ok(())
    }
}

impl Builder {
    /// Creates a new `Builder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    ///
    /// let builder = Builder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
            no_color: utils::no_color(),
            force_color: false,
        }
    }

    /// Appends text to the builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    ///
    /// let mut builder = Builder::new();
    /// builder.text("Hello, world!");
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Text("Hello, world!".to_string()));
    /// ```
    pub fn text(&mut self, text: &str) -> &mut Self {
        self.content.push(Entity::Text(text.to_string()));
        self
    }

    /// Prints the content of the `Builder`.
    /// This will print the content to stdout.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    ///
    /// let mut builder = Builder::new();
    /// builder.text("Hello, world!").bold().text(" This is bold text!").reset();
    /// builder.print();
    /// ```
    pub fn print(&self) {
        print!("{}", self);
    }

    /// Println the content of the `Builder`.
    /// This will print the content to stdout with newline.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    ///
    /// let mut builder = Builder::new();
    /// builder.text("Hello, world!").bold().text(" This is bold text!").reset();
    /// builder.println();
    /// ```
    pub fn println(&self) {
        println!("{}", self);
    }

    /// Returns the content of the `Builder` as a `String`.
    /// This will return the content as a `String` without any ANSI codes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    ///
    /// let mut builder = Builder::new();
    /// builder.text("Hello, world!").bold().text(" This is bold text!").reset();
    /// assert_eq!(builder.to_string(), "Hello, world! This is bold text!");
    /// ```
    pub fn to_string(&self) -> String {
        let mut content = String::new();
        if self.no_color && !self.force_color {
            for entity in &self.content {
                match entity {
                    Entity::Text(text) => content.push_str(text),
                    Entity::Ansi(_) => (),
                }
            }
            return content;
        }
        if self.content.get(self.content.len() - 1) != Some(&Entity::Ansi(Ansi::Reset)) {
            content.push_str(&Ansi::Reset.to_string());
        }
        for entity in &self.content {
            match entity {
                Entity::Text(text) => content.push_str(text),
                Entity::Ansi(ansi) => content.push_str(&ansi.to_string()),
            }
        }
        content
    }

    /// Returns the raw content of the `Builder`.
    /// This is useful for debugging purposes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    ///
    /// let mut builder = Builder::new();
    /// builder.text("Hello, world!");
    /// assert_eq!(builder.content_raw().len(), 1);
    /// ```
    pub fn content_raw(&self) -> Vec<Entity> {
        self.content.clone()
    }

    /// Appends an ANSI code to the builder.
    /// This is useful for custom ANSI codes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.ansi(Ansi::BgBlue);
    /// assert_eq!(builder.content_raw().len(), 1);
    pub fn ansi(&mut self, ansi: Ansi) -> &mut Self {
        self.content.push(Entity::Ansi(ansi));
        self
    }

    /// Appends a reset ANSI code.
    /// This will reset all styles and colors.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.reset();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::Reset));
    /// ```
    pub fn reset(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::Reset));
        self
    }

    /// Appends a bold ANSI code.
    /// This will make the text bold.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.bold();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::Bold));
    /// ```
    pub fn bold(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::Bold));
        self
    }

    /// Appends a faint ANSI code.
    /// This will make the text faint.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.faint();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::Faint));
    /// ```
    pub fn faint(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::Faint));
        self
    }

    /// Appends a italic ANSI code.
    /// This will make the text faint.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.italic();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::Italic));
    /// ```
    pub fn italic(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::Italic));
        self
    }

    /// Appends a underline ANSI code.
    /// This will make the text underlined.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.underline();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::Underline));
    /// ```
    pub fn underline(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::Underline));
        self
    }

    /// Appends a slow blink ANSI code.
    /// This will make the text blink slowly.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.blink_slow();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::BlinkSlow));
    /// ```
    pub fn blink_slow(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::BlinkSlow));
        self
    }

    /// Appends a rapid blink ANSI code.
    /// This will make the text blink rapidly.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.blink_rapid();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::BlinkRapid));
    /// ```
    pub fn blink_rapid(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::BlinkRapid));
        self
    }

    /// Appends a reverse video ANSI code.
    /// This will reverse the text and background colors.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.reverse_video();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::ReverseVideo));
    /// ```
    pub fn reverse_video(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::ReverseVideo));
        self
    }

    /// Appends a concealed ANSI code.
    /// This will hide the text.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.concealed();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::Concealed));
    /// ```
    pub fn concealed(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::Concealed));
        self
    }

    /// Appends a crossed out ANSI code.
    /// This will cross out the text.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.crossed_out();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::CrossedOut));
    /// ```
    pub fn crossed_out(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::CrossedOut));
        self
    }

    /// Appends a black foreground ANSI code.
    /// This will set the text color to black.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.fg_black();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::FgBlack));
    /// ```
    pub fn fg_black(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::FgBlack));
        self
    }

    /// Appends a red foreground ANSI code.
    /// This will set the text color to red.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.fg_red();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::FgRed));
    /// ```
    pub fn fg_red(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::FgRed));
        self
    }

    /// Appends a green foreground ANSI code.
    /// This will set the text color to green.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.fg_green();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::FgGreen));
    /// ```
    pub fn fg_green(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::FgGreen));
        self
    }

    /// Appends a yellow foreground ANSI code.
    /// This will set the text color to yellow.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.fg_yellow();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::FgYellow));
    /// ```
    pub fn fg_yellow(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::FgYellow));
        self
    }

    /// Appends a blue foreground ANSI code.
    /// This will set the text color to blue.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.fg_blue();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::FgBlue));
    /// ```
    pub fn fg_blue(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::FgBlue));
        self
    }

    /// Appends a magenta foreground ANSI code.
    /// This will set the text color to magenta.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.fg_magenta();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::FgMagenta));
    /// ```
    pub fn fg_magenta(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::FgMagenta));
        self
    }

    /// Appends a cyan foreground ANSI code.
    /// This will set the text color to cyan.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.fg_cyan();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::FgCyan));
    /// ```
    pub fn fg_cyan(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::FgCyan));
        self
    }

    /// Appends a white foreground ANSI code.
    /// This will set the text color to white.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.fg_white();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::FgWhite));
    /// ```
    pub fn fg_white(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::FgWhite));
        self
    }

    /// Appends a high intensity black foreground ANSI code.
    /// This will set the text color to high intensity black.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.fg_hi_black();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::FgHiBlack));
    /// ```
    pub fn fg_hi_black(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::FgHiBlack));
        self
    }

    /// Appends a high intensity red foreground ANSI code.
    /// This will set the text color to high intensity red.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.fg_hi_red();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::FgHiRed));
    /// ```
    pub fn fg_hi_red(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::FgHiRed));
        self
    }

    /// Appends a high intensity green foreground ANSI code.
    /// This will set the text color to high intensity green.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.fg_hi_green();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::FgHiGreen));
    /// ```
    pub fn fg_hi_green(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::FgHiGreen));
        self
    }

    /// Appends a high intensity yellow foreground ANSI code.
    /// This will set the text color to high intensity yellow.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.fg_hi_yellow();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::FgHiYellow));
    /// ```
    pub fn fg_hi_yellow(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::FgHiYellow));
        self
    }

    /// Appends a high intensity blue foreground ANSI code.
    /// This will set the text color to high intensity blue.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.fg_hi_blue();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::FgHiBlue));
    /// ```
    pub fn fg_hi_blue(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::FgHiBlue));
        self
    }

    /// Appends a high intensity magenta foreground ANSI code.
    /// This will set the text color to high intensity magenta.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.fg_hi_magenta();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::FgHiMagenta));
    /// ```
    pub fn fg_hi_magenta(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::FgHiMagenta));
        self
    }

    /// Appends a high intensity cyan foreground ANSI code.
    /// This will set the text color to high intensity cyan.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.fg_hi_cyan();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::FgHiCyan));
    /// ```
    pub fn fg_hi_cyan(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::FgHiCyan));
        self
    }

    /// Appends a high intensity white foreground ANSI code.
    /// This will set the text color to high intensity white.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.fg_hi_white();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::FgHiWhite));
    /// ```
    pub fn fg_hi_white(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::FgHiWhite));
        self
    }

    /// Appends a black background ANSI code.
    /// This will set the background color to black.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.bg_black();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::BgBlack));
    /// ```
    pub fn bg_black(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::BgBlack));
        self
    }

    /// Appends a red background ANSI code.
    /// This will set the background color to red.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.bg_red();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::BgRed));
    /// ```
    pub fn bg_red(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::BgRed));
        self
    }

    /// Appends a green background ANSI code.
    /// This will set the background color to green.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.bg_green();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::BgGreen));
    /// ```
    pub fn bg_green(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::BgGreen));
        self
    }

    /// Appends a yellow background ANSI code.
    /// This will set the background color to yellow.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.bg_yellow();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::BgYellow));
    /// ```
    pub fn bg_yellow(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::BgYellow));
        self
    }

    /// Appends a blue background ANSI code.
    /// This will set the background color to blue.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.bg_blue();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::BgBlue));
    /// ```
    pub fn bg_blue(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::BgBlue));
        self
    }

    /// Appends a magenta background ANSI code.
    /// This will set the background color to magenta.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.bg_magenta();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::BgMagenta));
    /// ```
    pub fn bg_magenta(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::BgMagenta));
        self
    }

    /// Appends a cyan background ANSI code.
    /// This will set the background color to cyan.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.bg_cyan();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::BgCyan));
    /// ```
    pub fn bg_cyan(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::BgCyan));
        self
    }

    /// Appends a white background ANSI code.
    /// This will set the background color to white.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.bg_white();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::BgWhite));
    /// ```
    pub fn bg_white(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::BgWhite));
        self
    }

    /// Appends a high intensity black background ANSI code.
    /// This will set the background color to high intensity black.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.bg_hi_black();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::BgHiBlack));
    /// ```
    pub fn bg_hi_black(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::BgHiBlack));
        self
    }

    /// Appends a high intensity red background ANSI code.
    /// This will set the background color to high intensity red.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.bg_hi_red();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::BgHiRed));
    /// ```
    pub fn bg_hi_red(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::BgHiRed));
        self
    }

    /// Appends a high intensity green background ANSI code.
    /// This will set the background color to high intensity green.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.bg_hi_green();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::BgHiGreen));
    /// ```
    pub fn bg_hi_green(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::BgHiGreen));
        self
    }

    /// Appends a high intensity yellow background ANSI code.
    /// This will set the background color to high intensity yellow.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.bg_hi_yellow();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::BgHiYellow));
    /// ```
    pub fn bg_hi_yellow(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::BgHiYellow));
        self
    }

    /// Appends a high intensity blue background ANSI code.
    /// This will set the background color to high intensity blue.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.bg_hi_blue();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::BgHiBlue));
    /// ```
    pub fn bg_hi_blue(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::BgHiBlue));
        self
    }

    /// Appends a high intensity magenta background ANSI code.
    /// This will set the background color to high intensity magenta.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.bg_hi_magenta();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::BgHiMagenta));
    /// ```
    pub fn bg_hi_magenta(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::BgHiMagenta));
        self
    }

    /// Appends a high intensity cyan background ANSI code.
    /// This will set the background color to high intensity cyan.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.bg_hi_cyan();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::BgHiCyan));
    /// ```
    pub fn bg_hi_cyan(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::BgHiCyan));
        self
    }

    /// Appends a high intensity white background ANSI code.
    /// This will set the background color to high intensity white.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcolors::builder::Builder;
    /// use rcolors::builder::Entity;
    /// use rcolors::ansi::Ansi;
    ///
    /// let mut builder = Builder::new();
    /// builder.bg_hi_white();
    /// assert_eq!(builder.content_raw().len(), 1);
    /// assert_eq!(builder.content_raw()[0], Entity::Ansi(Ansi::BgHiWhite));
    /// ```
    pub fn bg_hi_white(&mut self) -> &mut Self {
        self.content.push(Entity::Ansi(Ansi::BgHiWhite));
        self
    }
}

#[cfg(test)]
mod builder_tests {
    use std::io::Write;

    use super::*;
    use crate::ansi::Ansi;

    #[test]
    fn builder_new() {
        let builder = Builder::new();
        assert_eq!(builder.content.len(), 0);
        assert_eq!(builder.no_color, utils::no_color());
    }

    #[test]
    fn builder_text() {
        let mut builder = Builder::new();
        builder.text("Hello, world!");
        match &builder.content[0] {
            Entity::Text(text) => assert_eq!(text, "Hello, world!"),
            _ => panic!("Expected text entity"),
        }
    }

    #[test]
    fn builder_ansi() {
        let mut builder = Builder::new();
        builder.bold().italic();
        assert_eq!(builder.content.len(), 2);
        match &builder.content[0] {
            Entity::Ansi(Ansi::Bold) => (),
            _ => panic!("Expected Bold ANSI code"),
        }
        match &builder.content[1] {
            Entity::Ansi(Ansi::Italic) => (),
            _ => panic!("Expected Italic ANSI code"),
        }
    }

    #[test]
    fn display_with_color() {
        let mut builder = Builder::new();
        builder.force_color = true;
        builder.text("Hello, ").bold().text("world!").reset();
        assert_eq!(format!("{}", builder), "Hello, \x1b[1mworld!\x1b[0m");
    }

    #[test]
    fn display_without_color() {
        let mut builder = Builder::new();
        builder.no_color = true;
        builder.text("Hello, ").bold().text("world!").reset();
        assert_eq!(format!("{}", builder), "Hello, world!");
    }

    #[test]
    fn builder_reset() {
        let mut builder = Builder::new();
        builder.reset();
        assert_eq!(builder.content.len(), 1);
        match &builder.content[0] {
            Entity::Ansi(Ansi::Reset) => (),
            _ => panic!("Expected Reset ANSI code"),
        }
    }

    #[test]
    fn builder_fg_red() {
        let mut builder = Builder::new();
        builder.fg_red();
        assert_eq!(builder.content.len(), 1);
        match &builder.content[0] {
            Entity::Ansi(Ansi::FgRed) => (),
            _ => panic!("Expected FgRed ANSI code"),
        }
    }

    #[test]
    fn builder_bg_blue() {
        let mut builder = Builder::new();
        builder.bg_blue();
        assert_eq!(builder.content.len(), 1);
        match &builder.content[0] {
            Entity::Ansi(Ansi::BgBlue) => (),
            _ => panic!("Expected BgBlue ANSI code"),
        }
    }

    #[test]
    fn builder_combined_styles() {
        let mut builder = Builder::new();
        builder.bold().italic().underline();
        assert_eq!(builder.content.len(), 3);
        match &builder.content[0] {
            Entity::Ansi(Ansi::Bold) => (),
            _ => panic!("Expected Bold ANSI code"),
        }
        match &builder.content[1] {
            Entity::Ansi(Ansi::Italic) => (),
            _ => panic!("Expected Italic ANSI code"),
        }
        match &builder.content[2] {
            Entity::Ansi(Ansi::Underline) => (),
            _ => panic!("Expected Underline ANSI code"),
        }
    }

    #[test]
    fn builder_print() {
        let mut builder = Builder::new();
        builder.force_color = true;
        builder.text("Hello, ").bold().text("world!").reset();
        // Redirect stdout for testing
        let output = std::io::stdout();
        let mut handle = output.lock();
        builder.print();
        assert_eq!(handle.write(b"Hello, \x1b[1mworld!\x1b[0m").is_ok(), true);
    }

    #[test]
    fn builder_println() {
        let mut builder = Builder::new();
        builder.force_color = true;
        builder.text("Hello, ").bold().text("world!").reset();
        // Redirect stdout for testing
        let output = std::io::stdout();
        let mut handle = output.lock();
        builder.println();
        assert_eq!(handle.write(b"Hello, \x1b[1mworld!\x1b[0m\n").is_ok(), true);
    }

    #[test]
    fn builder_to_string() {
        let mut builder = Builder::new();
        builder.force_color = true;
        builder.text("Hello, ").bold().text("world!").reset();
        assert_eq!(builder.to_string(), "Hello, \u{1b}[1mworld!\u{1b}[0m");
    }
}
