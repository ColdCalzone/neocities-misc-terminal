fn run() -> Box<dyn FnOnce(Vec<String>, Receiver<SessionMessage>, Sender<SessionMessage>)> {
    use crate::key_events::*;
    use crate::terminal::{
        style::{Color, Span},
        ShellMessage, TerminalMessage,
    };
    Box::new(move |args, receiver, sender| {
        sender
            .send(SessionMessage::KillSessionYesReallyTheActualSessionNotSomeInternalThing)
            .unwrap();
    })
}
