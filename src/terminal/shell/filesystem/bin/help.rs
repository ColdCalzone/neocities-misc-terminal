fn run() -> Box<dyn FnOnce(Vec<String>, Receiver<SessionMessage>, Sender<SessionMessage>)> {
    use crate::key_events::*;
    use crate::terminal::{ShellMessage, TerminalMessage};
    Box::new(move |args, receiver, sender| {
        sender
            .send(SessionMessage::Terminal(
                TerminalMessage::Push("CASH v".into()),
                None,
            ))
            .unwrap();

        sender
            .send(SessionMessage::Terminal(
                TerminalMessage::PushLine("0.0.0.1".into()),
                None,
            ))
            .unwrap();

        sender
            .send(SessionMessage::Terminal(
                TerminalMessage::PushLine("Cold's Awful SHell".into()),
                None,
            ))
            .unwrap();
        sender
            .send(SessionMessage::Terminal(
                TerminalMessage::PushLine("No help to provide...".into()),
                None,
            ))
            .unwrap();

        sender
            .send(SessionMessage::Terminal(
                TerminalMessage::PushLine("Press enter to continue.".into()),
                None,
            ))
            .unwrap();

        sender
            .send(SessionMessage::Terminal(TerminalMessage::ForceUpdate, None))
            .unwrap();

        loop {
            match receiver.recv() {
                Ok(SessionMessage::Shell(ShellMessage::InputKeyEvent(k), _)) => {
                    if let Key::Enter = k.key_type {
                        break;
                    }
                }
                Err(_) => {
                    break;
                }
                _ => {}
            }
        }
    })
}
