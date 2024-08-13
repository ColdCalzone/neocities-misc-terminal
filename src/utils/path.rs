use std::{
    fmt,
    ops::{Deref, DerefMut},
};

pub struct Path(Vec<String>);

impl Path {
    pub fn new(path: &str) -> Self {
        Path(
            path.split('/')
                .map(|x| x.to_owned())
                .collect::<Vec<String>>(),
        )
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/{}", self.join("/"))
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        Path::new(value)
    }
}

impl Deref for Path {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Path {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
