use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use tree::*;

// See: src/terminal/shell/files.rs
pub enum FileType {
    Program(String),
    Binary(Vec<u8>),
}

impl FileType {
    fn instructions(&self) -> String {
        match self {
            Self::Program(x) => {
                format!("FileType::Program({})", x)
            }
            Self::Binary(x) => {
                format!(
                    "FileType::Binary(vec![{}])",
                    x.iter()
                        .map(|x| format!("{x}"))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }
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
    fn instructions(&self) -> String {
        match self {
            Self::File { name, contents } => {
                format!(
                    "FSObject::File {{
    name: \"{}\".into(),
    contents: {}
}}",
                    name,
                    contents.instructions()
                )
            }
            Self::Folder { name, contents: _ } => {
                format!(
                    "FSObject::Folder {{
    name: \"{}\".into(),
    contents: HashMap::new()
}}",
                    name
                )
            }
        }
    }
}

fn read_filesystem(filesystem: &mut Tree<FSObject>, dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let mut child = Tree::new(FSObject::Folder {
                    name: String::from(path.file_name().unwrap().to_str().unwrap()),
                    contents: HashMap::new(),
                });
                read_filesystem(&mut child, &path)?;
                filesystem.insert_child(child);
            } else {
                let ext = path.extension();
                if let Some(os_str) = ext {
                    if os_str == "rs" {
                        let child = Tree::new(FSObject::File {
                            name: String::from(
                                path.with_extension("")
                                    .file_name()
                                    .unwrap()
                                    .to_str()
                                    .unwrap(),
                            ),
                            contents: FileType::Program(format!(
                                "{{
                                    {}

                                    run
                                }}",
                                fs::read_to_string(path)?
                            )),
                        });
                        filesystem.insert_child(child);

                        continue;
                    }
                }
                let child = Tree::new(FSObject::File {
                    name: String::from(path.file_name().unwrap().to_str().unwrap()),
                    contents: FileType::Binary(fs::read(path)?),
                });
                filesystem.insert_child(child);
            }
        }
    }
    Ok(())
}

fn reconstruct(reconstruction: &mut String, tree: &Tree<'_, FSObject>) {
    let fsobj = tree.get_value();
    let mut as_child = true;
    if let FSObject::Folder { ref name, .. } = *fsobj {
        if name == "/" {
            as_child = false;
        }
    }

    if as_child {
        reconstruction.push_str(
            "
.with_child(
",
        )
    }

    reconstruction.push_str(&format!("Tree::new({})", fsobj.instructions()));

    tree.bfs_iter().max_depth(1).skip_root().for_each(|x| {
        reconstruct(reconstruction, x);
        reconstruction.push(')');
    })
}

fn main() -> io::Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    // let out_dir: String = "./".into();
    let dest_path = Path::new(&out_dir).join("filesystem.tree");

    let mut filesystem: Tree<FSObject> = Tree::new(FSObject::Folder {
        name: "/".into(),
        contents: HashMap::new(),
    });

    read_filesystem(&mut filesystem, Path::new("src/terminal/shell/filesystem")).ok();

    let mut reconstruction: String = String::new();

    if !filesystem.is_empty() {
        reconstruct(&mut reconstruction, &filesystem);
    }

    fs::write(dest_path, reconstruction)?;

    Ok(())
}
