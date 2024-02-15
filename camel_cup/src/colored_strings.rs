pub mod color;
pub mod format;

pub use color::*;
pub use format::*;

#[derive(Clone, Debug)]
pub struct ColoredString {
    pub format: Format,
    pub text: String,
}

#[macro_export]
macro_rules! Color {
    () => {
        ColoredString::new("")
    };
    ($input:expr) => {
        ColoredString::new_string(format!("{}", $input))
    };
}

impl ColoredString {
    pub fn new(text: &str) -> ColoredString {
        ColoredString {
            format: Format::new(),
            text: text.to_string(),
        }
    }
    pub fn new_string(text: String) -> ColoredString {
        ColoredString {
            format: Format::new(),
            text,
        }
    }
    pub fn foreground(mut self, color: &Color) -> ColoredString {
        self.format.foreground = Some(color.clone());
        self
    }
    pub fn background(mut self, color: &Color) -> ColoredString {
        self.format.background = Some(color.clone());
        self
    }
    pub fn bold(mut self, bold: bool) -> ColoredString {
        self.format.bold = Some(bold);
        self
    }
    pub fn underline(mut self, underline: bool) -> ColoredString {
        self.format.underline = Some(underline);
        self
    }
    pub fn blink(mut self, blink: bool) -> ColoredString {
        self.format.blink = Some(blink);
        self
    }
    pub fn reverse(mut self, reverse: bool) -> ColoredString {
        self.format.reverse = Some(reverse);
        self
    }
    pub fn hidden(mut self, hidden: bool) -> ColoredString {
        self.format.hidden = Some(hidden);
        self
    }
}

pub(crate) trait PushColoredString {
    fn push_colored(&mut self, colored: ColoredString);
}

impl PushColoredString for String {
    fn push_colored(&mut self, colored: ColoredString) {
        match colored.format.reset_at_end {
            true => self.push_str(&format!("{}{}{}", colored.format.to_escape_codes(), colored.text, "\x1B[0m")),
            false => self.push_str(&format!("{}{}", colored.format.to_escape_codes(), colored.text)),
        }
    }
}

use std::fmt::{Display, Formatter, Result};
impl Display for ColoredString {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.format.reset_at_end {
            true => write!(f, "{}{}{}", self.format.to_escape_codes(), self.text, "\x1B[0m"),
            false => write!(f, "{}{}", self.format.to_escape_codes(), self.text),            
        }
    }
}