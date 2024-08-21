use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, LazyLock, RwLock};
use std::{borrow::Borrow, sync::mpsc::Sender};
use tree::send_tree::*;

use crate::session::SessionMessage;

pub static FILESYSTEM: LazyLock<SendTree<FSObject>> = LazyLock::new(|| SendTree::new_filesystem());

pub type Program = Box<dyn FnOnce(Vec<String>, Receiver<SessionMessage>, Sender<SessionMessage>)>;
pub enum FileType {
    Program(fn() -> Program),
    Binary(Vec<u8>),
}

pub enum FSObject {
    File {
        name: String,
        contents: FileType,
    },
    Folder {
        name: String,
        contents: HashMap<String, usize>, // Lookup table for child indices
    },
    // Symlink(Weak<Node<FSObject>>),
}

impl FSObject {
    pub fn get_permissions(&self) -> Option<()> {
        match self {
            Self::File { name, contents } => todo!(),
            Self::Folder { name, contents } => todo!(),
        }
        None
    }

    pub fn get_name(&self) -> &String {
        match self {
            Self::File { name, contents: _ } => name,
            Self::Folder { name, contents: _ } => name,
        }
    }

    pub fn is_folder(&self) -> bool {
        matches!(*self, Self::Folder { .. })
    }

    pub fn is_file(&self) -> bool {
        matches!(*self, Self::File { .. })
    }
}

pub type AsyncFSObject = Arc<RwLock<FSObject>>;

pub trait FileSystem<'a> {
    fn new_root() -> Self;

    fn new_filesystem() -> Self;

    fn get_by_path(&'a self, path: &Path) -> Option<AsyncFSObject>;

    fn index_children(&mut self);

    fn indexed(self) -> Self;
}

impl<'a> FileSystem<'a> for SendTree<'a, FSObject> {
    fn new_root() -> Self {
        SendTree::new(FSObject::Folder {
            name: "/".into(),
            contents: HashMap::new(),
        })
    }

    fn new_filesystem() -> Self {
        include!(concat!(env!("OUT_DIR"), "/filesystem.tree"))
    }

    fn get_by_path(&'a self, path: &Path) -> Option<AsyncFSObject> {
        let mut out: Option<&SendTree<FSObject>> = Some(self);
        let mut components = path.components();
        components.next(); // remove "/"
        for obj in components {
            out = out.and_then(|current_obj| {
                let ref_arc = current_obj.get_value();
                let fsobj = ref_arc.read().expect("Couldn't get lock on folder");
                if let FSObject::Folder {
                    name: _,
                    ref contents,
                } = *fsobj
                {
                    return contents
                        .get(obj.as_os_str().to_str().unwrap())
                        .and_then(|index| current_obj.get_child(*index).borrow().to_owned());
                }

                None
            })
        }
        out.map(|x| x.get_value().clone())
    }

    fn index_children(&mut self) {
        let fs_ptr = self.get_value();
        let mut fs_ref = (fs_ptr).write().expect("Couldn't get write access to file");
        if fs_ref.is_folder() {
            let mut i = 0usize;
            let contents = match *fs_ref {
                FSObject::File { .. } => unreachable!(),
                FSObject::Folder {
                    ref mut contents, ..
                } => contents,
            };
            contents.clear();
            while let Some(child) = self.get_child(i) {
                let child_ref = child.get_value();
                let x = child_ref.read().expect("Couldn't get read access to file");
                let name = x.get_name().clone();
                contents.insert(name, i);
                i += 1;
            }
        }
    }

    fn indexed(mut self) -> Self {
        self.index_children();
        self
    }
}
