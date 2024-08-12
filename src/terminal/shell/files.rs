// use std::rc::Weak;

use crate::utils::tree::*;

pub enum FSObject {
    File { name: String, contents: Vec<u8> },
    Folder { name: String },
    // Symlink(Weak<Node<FSObject>>),
}

pub type FileSystem = Tree<FSObject>;

impl FileSystem {
    pub fn new_root() -> Self {
        FileSystem::new(FSObject::Folder { name: "/".into() })
    }
}
