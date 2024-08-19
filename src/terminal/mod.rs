use crate::utils::pos::Size;
use std::{
    fmt,
    sync::{Arc, Mutex},
};

pub mod shell;

pub struct Terminal {
    buffer: Arc<Mutex<Vec<String>>>,
    scroll: usize,
    size: Option<Size>,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
            scroll: 0,
            size: None,
        }
    }

    pub fn set_size(&mut self, rect: Size) {
        self.size = Some(rect);
    }

    pub fn get_buffer(&mut self) -> Arc<Mutex<Vec<String>>> {
        self.buffer.clone()
    }

    fn clear(&self) {
        let lock = (*self.buffer).lock();
        match lock {
            Ok(mut x) => {
                x.clear();
            }
            Err(e) => {
                panic!("terminal clear: Couldn't get write access to buffer: {e}");
            }
        }
    }
}

impl fmt::Display for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lock = self.buffer.lock();
        match lock {
            Ok(x) => write!(f, "{}", x.join("\n")),
            Err(e) => panic!("Displaying terminal: Couldn't lock buffer: {e}"),
        }
    }
}
