Tree::new(FSObject::Folder {
                        name: "/".into(),
                        contents: HashMap::new()
                    })
.with_child(
Tree::new(FSObject::Folder {
                        name: "home".into(),
                        contents: HashMap::new()
                    })
.with_child(
Tree::new(FSObject::Folder {
                        name: "guest".into(),
                        contents: HashMap::new()
                    })))
.with_child(
Tree::new(FSObject::Folder {
                        name: "bin".into(),
                        contents: HashMap::new()
                    })
.with_child(
Tree::new(FSObject::File {
                    name: "help".into(),
                    contents: FileType::Program({
                                    fn run() {
    println!("Help will not come.");
}


                                    run
                                })
                })))