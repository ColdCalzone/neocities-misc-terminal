use std::rc::Weak;

use crate::utils::tree::*;

enum FSObject {
    File,
    Folder,
    Symlink(Weak<FSObject>),
}
