use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Color(u8, u8, u8);

impl Color {
    pub fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    pub fn new_rgb_tuple(rgb: (u8, u8, u8)) -> Self {
        Self::new_rgb(rgb.0, rgb.1, rgb.2)
    }

    fn r(&self) -> u8 {
        self.0
    }

    fn g(&self) -> u8 {
        self.1
    }

    fn b(&self) -> u8 {
        self.2
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Decoration {
    None = 0,
    Bold = 1,
    Italic = 2,
    Underline = 4,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Default)]
pub struct Span {
    pub text: String,
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
    pub decoration: u8,
}

impl Span {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            fg_color: None,
            bg_color: None,
            decoration: 0,
        }
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn with_fg_color(mut self, color: Color) -> Self {
        self.fg_color = Some(color);
        self
    }

    pub fn with_bg_color(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    pub fn bold(mut self) -> Self {
        self.decoration ^= Decoration::Bold as u8;
        self
    }

    pub fn italic(mut self) -> Self {
        self.decoration ^= Decoration::Italic as u8;
        self
    }

    pub fn underline(mut self) -> Self {
        self.decoration ^= Decoration::Underline as u8;
        self
    }
}

impl Display for Span {
    #[cfg(target_arch = "wasm32")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {}
    #[cfg(not(target_arch = "wasm32"))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.decoration & Decoration::Bold as u8 != 0 {
            write!(f, "\x1b[1m")?;
        }
        if self.decoration & Decoration::Italic as u8 != 0 {
            write!(f, "\x1b[3m")?;
        }
        if self.decoration & Decoration::Underline as u8 != 0 {
            write!(f, "\x1b[4m")?;
        }
        if let Some(fg) = self.fg_color {
            write!(f, "\x1b[38;2;{};{};{}m", fg.r(), fg.g(), fg.b())?;
        }
        if let Some(bg) = self.bg_color {
            write!(f, "\x1b[48;2;{};{};{}m", bg.r(), bg.g(), bg.b())?;
        }
        write!(f, "{}\x1b[0m", self.text)
    }
}

impl From<&str> for Span {
    fn from(value: &str) -> Self {
        Self::new().with_text(value.into())
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug)]
pub struct SpanSet(Vec<Span>);

impl SpanSet {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Deref for SpanSet {
    type Target = Vec<Span>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SpanSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for SpanSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for span in &self.0 {
            write!(f, "{span}")?;
        }
        Ok(())
    }
}
