fn run() -> Box<dyn FnOnce(Vec<String>, Receiver<SessionMessage>, Sender<SessionMessage>)> {
    use crate::key_events::*;
    use crate::terminal::{
        style::{Color, Span},
        ShellMessage, TerminalMessage,
    };
    use std::sync::mpsc::TryRecvError;

    Box::new(move |args, receiver, sender| {
        let cash: [Span; 4] = [
            Span::new()
                .with_fg_color(Color::new_rgb(225, 30, 50))
                .with_text("C".into()),
            Span::new()
                .with_fg_color(Color::new_rgb(190, 190, 30))
                .with_text("A".into()),
            Span::new()
                .with_fg_color(Color::new_rgb(30, 225, 80))
                .with_text("$".into()),
            Span::new()
                .with_fg_color(Color::new_rgb(40, 90, 190))
                .with_text("H".into()),
        ];

        for letter in cash {
            sender
                .send(SessionMessage::Terminal(
                    TerminalMessage::PushSpan(letter),
                    None,
                ))
                .unwrap();
        }

        sender
            .send(SessionMessage::Terminal(
                TerminalMessage::PushSpan(Span::new()),
                None,
            ))
            .unwrap();

        sender
            .send(SessionMessage::Terminal(
                TerminalMessage::Push(" v".into()),
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
                TerminalMessage::Push("Press enter to continue.".into()),
                None,
            ))
            .unwrap();

        sender
            .send(SessionMessage::Terminal(TerminalMessage::ForceUpdate, None))
            .unwrap();

        loop {
            match receiver.try_recv() {
                Ok(m) => {
                    if let SessionMessage::Shell(ShellMessage::InputKeyEvent(k), _) = m {
                        if let Key::Enter = k.key_type {
                            sender
                                .send(SessionMessage::Terminal(
                                    TerminalMessage::Push("\r\n".into()),
                                    None,
                                ))
                                .unwrap();
                            break;
                        }
                    }
                }
                Err(TryRecvError::Disconnected) => {
                    break;
                }
                _ => {}
            }
        }
        sender
            .send(SessionMessage::Shell(ShellMessage::ExitCode(0), None))
            .unwrap();
    })
}
