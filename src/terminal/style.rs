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

pub struct Span {
    pub text: String,
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
}
