use std::fmt;

pub struct Path {
    path: Vec<String>,
}

impl Path {
    fn new(path: &str) -> Self {
        Path {
            path: path
                .split('/')
                .map(|x| x.to_owned())
                .collect::<Vec<String>>(),
        }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/{}", self.path.join("/"))
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        Path::new(value)
    }
}
