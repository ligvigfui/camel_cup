use crate::*;

#[derive(Clone, Debug)]
pub struct Format {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub bold: Option<bool>,
    pub underline: Option<bool>,
    pub blink: Option<bool>,
    pub reverse: Option<bool>,
    pub hidden: Option<bool>,
    pub reset_at_end: bool,
}
impl Format {
    pub fn new() -> Format {
        Format {
            foreground: None,
            background: None,
            bold: None,
            underline: None,
            blink: None,
            reverse: None,
            hidden: None,
            reset_at_end: true,
        }
    }
    pub fn to_escape_codes(&self) -> String {
        let mut result = String::new();
        if let Some(foreground) = &self.foreground {
            result.push_str(&foreground.to_foreground());
        }
        if let Some(background) = &self.background {
            result.push_str(&background.to_background());
        }
        if let Some(bold) = self.bold {
            match bold {
                true => result.push_str("\x1B[1m"),
                false => result.push_str("\x1B[22m"),
        }}
        if let Some(underline) = self.underline {
            match underline {
                true => result.push_str("\x1B[4m"),
                false => result.push_str("\x1B[24m"),
        }}
        if let Some(blink) = self.blink {
            match blink {
                true => result.push_str("\x1B[5m"),
                false => result.push_str("\x1B[25m"),
        }}
        if let Some(reverse) = self.reverse {
            match reverse {
                true => result.push_str("\x1B[7m"),
                false => result.push_str("\x1B[27m"),
        }}
        if let Some(hidden) = self.hidden {
            match hidden {
                true => result.push_str("\x1B[8m"),
                false => result.push_str("\x1B[28m"),
        }}
        result
    }
}