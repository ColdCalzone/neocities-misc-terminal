use crate::utils::rect::Rectangle;
use std::{fmt, sync::RwLock};

pub mod key_event;
pub mod shell;

pub struct Terminal {
    buffer: RwLock<Vec<String>>,
    scroll: usize,
    size: Option<Rectangle>,
}

impl Terminal {
    pub fn set_rect(mut self, rect: Rectangle) -> Self {
        self.size = Some(rect);
        self
    }

    fn clear(&self) {
        let mut buff = self
            .buffer
            .write()
            .expect("terminal clear: Couldn't get write access to buffer");
        (*buff).clear();
    }
}

impl fmt::Display for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = self
            .buffer
            .read()
            .expect("Displaying terminal: Couldn't get read access to buffer")
            .join("\n");

        write!(f, "{}", out)
    }
}
