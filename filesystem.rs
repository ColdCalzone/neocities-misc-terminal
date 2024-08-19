SendTree::new(FSObject::Folder {
    name: "/".into(),
    contents: HashMap::new()
})
.with_child(
SendTree::new(FSObject::Folder {
    name: "home".into(),
    contents: HashMap::new()
})
.with_child(
SendTree::new(FSObject::Folder {
    name: "guest".into(),
    contents: HashMap::new()
})
.with_child(
SendTree::new(FSObject::File {
    name: "README.md".into(),
    contents: FileType::Binary(vec![35, 32, 109, 105, 115, 99, 47, 32, 116, 101, 114, 109, 105, 110, 97, 108, 10, 10, 84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 40, 114, 97, 116, 104, 101, 114, 32, 111, 118, 101, 114, 101, 110, 103, 105, 110, 101, 101, 114, 101, 100, 41, 32, 116, 101, 114, 109, 105, 110, 97, 108, 32, 102, 111, 114, 32, 109, 121, 32, 91, 110, 101, 111, 99, 105, 116, 105, 101, 115, 32, 112, 97, 103, 101, 93, 40, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 67, 111, 108, 100, 67, 97, 108, 122, 111, 110, 101, 47, 110, 101, 111, 99, 105, 116, 105, 101, 115, 41, 46, 10])
}).indexed()).indexed()).indexed())
.with_child(
SendTree::new(FSObject::Folder {
    name: "bin".into(),
    contents: HashMap::new()
})
.with_child(
SendTree::new(FSObject::File {
    name: "help".into(),
    contents: FileType::Program({
                        fn run() -> fn(Vec<String>, Receiver<SessionMessage>, Sender<SessionMessage>) {
    move |args, _r, sender| {
        sender
            .send(SessionMessage::Output(OutputMessage::PushLine(
                "CASH v 0.0.0",
            )))
            .unwrap();
        sender
            .send(SessionMessage::Output(OutputMessage::PushLine(
                "Cold's Awful SHell",
            )))
            .unwrap();
        sender
            .send(SessionMessage::Output(OutputMessage::PushLine(
                "No help to provide...",
            )))
            .unwrap();
    }
}


                        run
                    })
}).indexed())
.with_child(
SendTree::new(FSObject::File {
    name: "cash".into(),
    contents: FileType::Program({
                        // CASH -- Cold's Awful SHell
// Name derived from CA$H, the R.A.M Demo's (second) hardest challenge.
fn run() -> fn(Vec<String>, Receiver<SessionMessage>, Sender<SessionMessage>) {
    use crate::key_events::*;
    let mut input: String = String::new();

    fn get_prefix(shell_tx: &mut Sender<SessionMessage>) -> String {
        //     let path = match self.cwd.lock() {
        //         Ok(x) => x,
        //         Err(e) => panic!("Error getting input prefix: {e}"),
        //     };
        //     format!(
        //         "[{}@deep-freezer:{}]$",
        //         self.user.get_name(),
        //         path.to_string_lossy()
        // )
    }

    move |args, events, shell_tx| loop {
        match events.recv() {
            SessionMessage::InputKeyEvent(key_event) => match key_event.key_type {
                Key::Char { data } => {
                    if key_event.modifier == Modifier::Shift {
                        input.push(data.to_ascii_uppercase());
                    } else {
                        input.push(data.to_ascii_lowercase());
                    }
                }
                Key::Backspace => {
                    input.pop();
                }
                _ => {}
            },
            Interrupt => break,
            _ => {}
        }
    }
}


                        run
                    })
}).indexed()).indexed())
.with_child(
SendTree::new(FSObject::Folder {
    name: "etc".into(),
    contents: HashMap::new()
})
.with_child(
SendTree::new(FSObject::File {
    name: "profile".into(),
    contents: FileType::Binary(vec![47, 98, 105, 110, 47, 10])
}).indexed()).indexed()).indexed()