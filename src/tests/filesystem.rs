use crate::terminal::shell::files::*;
use tree::Tree;

#[test]
fn filesystem() {
    let filesystem: Tree<FSObject> = Tree::new_filesystem();

    filesystem.dfs_iter().for_each(|x| match *(x.get_value()) {
        FSObject::File {
            ref name,
            ref contents,
        } => {
            println!("File \"{name}\"");
            match contents {
                FileType::Program(x) => {
                    println!("Running {name}:");
                    x();
                }
                FileType::Binary(v) => {
                    println!("{}: {}", name, String::from_utf8_lossy(v));
                }
            };
        }
        FSObject::Folder {
            ref name,
            contents: _,
        } => {
            println!("Folder \"{name}\":");
        }
    });

    assert_eq!(
        filesystem.get_child(0).and_then(|x| {
            if let FSObject::Folder { ref name, .. } = *x.get_value() {
                Some(name.clone())
            } else {
                None
            }
        }),
        Some(String::from("home"))
    )
}
