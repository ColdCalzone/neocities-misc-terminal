fn run() -> Box<dyn FnOnce(Vec<String>, Receiver<SessionMessage>, Sender<SessionMessage>)> {
    use crate::key_events::*;
    use crate::terminal::{
        style::{Color, Span},
        ShellMessage, TerminalMessage,
    };

    Box::new(move |args, receiver, sender| {
        if cfg!(target_arch = "wasm32") && args.len() >= 2 {
            let window = web_sys::window().expect("no `window` exists!");
            let location = window.location();
            location
                .set_href(&format!("https://{}.neocities.org/", args[1]))
                .unwrap();
        } else {
            sender
                .send(SessionMessage::Terminal(
                    TerminalMessage::PushSpan("Unsupported command.\r\n".into()),
                    None,
                ))
                .unwrap();
        }

        sender
            .send(SessionMessage::Shell(ShellMessage::ExitCode(0), None))
            .unwrap();
    })
}
