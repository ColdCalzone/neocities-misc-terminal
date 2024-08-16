use private::FileSystemPrivate;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;
use tree::*;

pub enum FileType {
    Program(fn() -> ()),
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

pub(crate) mod private {
    use tree::{RefMut, Tree};

    use super::FSObject;

    pub trait FileSystemPrivate<'a> {
        fn index_children(&'a mut self);
    }

    impl<'a> FileSystemPrivate<'a> for Tree<'a, FSObject> {
        fn index_children(&'a mut self) {
            let fs_ptr = self.get_value_pointer();
            let fs_ref = fs_ptr.borrow_mut();
            if fs_ref.is_folder() {
                let mut i = 0usize;
                let mut contents = RefMut::map(fs_ref, |f| match f {
                    FSObject::File { .. } => unreachable!(),
                    FSObject::Folder { contents, .. } => contents,
                });
                contents.clear();
                while let Some(child) = self.get_child(i) {
                    let x = child.get_value();
                    let name = x.get_name().clone();
                    (*contents).insert(name, i);
                    i += 1;
                }
            }
        }
    }
}

pub trait FileSystem<'a>: private::FileSystemPrivate<'a> {
    fn new_root() -> Self;

    fn get_by_path(&'a self, path: &Path) -> Option<Ref<FSObject>>;
}

impl<'a> FileSystem<'a> for Tree<'a, FSObject> {
    fn new_root() -> Self {
        Tree::new(FSObject::Folder {
            name: "/".into(),
            contents: HashMap::new(),
        })
    }

    fn get_by_path(&'a self, path: &Path) -> Option<Ref<FSObject>> {
        let mut out: Option<&Tree<FSObject>> = Some(self);
        for obj in path.components() {
            out = out.and_then(|current_obj| {
                if let FSObject::Folder { name: _, contents } = current_obj.get_value().deref() {
                    return contents
                        .get(obj.as_os_str().to_str().unwrap())
                        .and_then(|index| current_obj.get_child(*index).borrow().to_owned());
                }
                None
            })
        }
        out.map(|x| x.get_value())
    }
}
