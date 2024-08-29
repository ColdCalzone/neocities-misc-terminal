// CASH -- Cold's Awful SHell
// Name derived from CA$H, the R.A.M Demo's (second) hardest challenge.
fn run() -> Box<dyn FnOnce(Vec<String>, Receiver<SessionMessage>, Sender<SessionMessage>)> {
    use crate::key_events::*;
    use crate::session::SessionMessage;
    use crate::session::{ReturnValue, ShellMessage, TerminalMessage};
    use crate::terminal::style::{Color, Span};
    use std::sync::mpsc::channel;
    use std::sync::mpsc::TryRecvError;
    use std::thread::{self, JoinHandle};

    struct CashProcess {
        pub thread: JoinHandle<()>,
        pub sender: Sender<SessionMessage>,
    }

    struct CashShellData {
        input: String,
        cwd: String,
        running: Option<CashProcess>,
    }

    #[derive(Clone, Copy, Debug)]
    enum CashState {
        /// Display MOTD, for example
        Initial,
        /// Getting user input
        Input,
        /// Processing what the user inputed
        Evaluating,
        /// Running a program
        Executing,
        /// Being told to blow up
        Interrupting,
    }

    fn get_prefix(shell_tx: &Sender<SessionMessage>, data: &CashShellData) -> Span {
        let path = &data.cwd;

        let (tx, rx) = channel();

        shell_tx
            .send(SessionMessage::Shell(
                ShellMessage::GetCurrentUser,
                Some(tx),
            ))
            .unwrap();

        if let SessionMessage::Return(ReturnValue::User(Some(user))) =
            rx.recv().expect("Couldn't get user for CASH prefix")
        {
            Span::new()
                .with_text(format!(
                    "[{}@deep-freezer:{}]$ ",
                    user.get_name(),
                    &path.replacen(format!("/home/{}", user.get_name()).as_str(), "~", 1)
                ))
                .with_fg_color(Color::new_rgb(20, 160, 190))
                .bold()
        } else {
            unreachable!("get_prefix did not receive a return message");
        }
    }

    fn state_transition(
        shell_tx: &Sender<SessionMessage>,
        state: Option<CashState>,
        data: &CashShellData,
        new_state: CashState,
    ) -> Option<CashState> {
        match (state, new_state) {
            (_, CashState::Input) => {
                shell_tx
                    .send(SessionMessage::Terminal(
                        TerminalMessage::PushSpan(get_prefix(shell_tx, &data)),
                        None,
                    ))
                    .unwrap();
                shell_tx
                    .send(SessionMessage::Terminal(
                        TerminalMessage::PushSpan(Span::new()),
                        None,
                    ))
                    .unwrap();
                shell_tx
                    .send(SessionMessage::Terminal(TerminalMessage::ForceUpdate, None))
                    .unwrap();
                Some(new_state)
            }
            _ => Some(new_state),
        }
    }

    fn initial_state(
        shell_tx: &Sender<SessionMessage>,
        state: &mut Option<CashState>,
        data: &mut CashShellData,
        events: &Receiver<SessionMessage>,
    ) -> () {
        shell_tx
            .send(SessionMessage::Terminal(
                TerminalMessage::PushSpan(
                    Span::new()
                        .with_text("Welcome to CA$H shell!\r\n".into())
                        .with_fg_color(Color::new_rgb(255, 255, 255))
                        .italic(),
                ),
                None,
            ))
            .unwrap();

        *state = state_transition(shell_tx, *state, data, CashState::Input);
    }

    fn input_state(
        shell_tx: &Sender<SessionMessage>,
        state: &mut Option<CashState>,
        data: &mut CashShellData,
        events: &Receiver<SessionMessage>,
    ) -> () {
        match events.recv() {
            Ok(SessionMessage::Shell(ShellMessage::InputKeyEvent(key_event), _)) => {
                match key_event.key_type {
                    Key::Char(ch) => {
                        if let Some(Modifier::Shift) = key_event.modifier {
                            data.input.push(ch.to_ascii_uppercase());
                        } else {
                            data.input.push(ch.to_ascii_lowercase());
                        }
                    }
                    Key::Backspace => {
                        data.input.pop();
                    }
                    Key::Enter => {
                        shell_tx
                            .send(SessionMessage::Terminal(
                                TerminalMessage::SetSpan(0, data.input.as_str().into()),
                                None,
                            ))
                            .unwrap();
                        shell_tx
                            .send(SessionMessage::Terminal(
                                TerminalMessage::PushSpan("\r\n".into()),
                                None,
                            ))
                            .unwrap();
                        *state = state_transition(shell_tx, *state, data, CashState::Evaluating);
                        return;
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        shell_tx
            .send(SessionMessage::Terminal(
                TerminalMessage::SetSpan(0, data.input.as_str().into()),
                None,
            ))
            .unwrap();

        shell_tx
            .send(SessionMessage::Terminal(TerminalMessage::ForceUpdate, None))
            .unwrap();
    }

    enum ParsingState {
        Normal,
        String,
    }

    fn args_parser(raw_input: &str) -> Result<Vec<String>, ()> {
        let mut state = ParsingState::Normal;
        let mut out = Vec::new();
        let mut current = String::new();

        let mut iter = raw_input.chars().peekable();

        while let Some(c) = iter.next() {
            match (c, &state) {
                (' ', ParsingState::Normal) => {
                    out.push(current);
                    current = String::new();
                }
                (' ', ParsingState::String) => {
                    current.push(c);
                }
                ('\\', _) => {
                    current.push(c);
                    if let Some(ch) = iter.next() {
                        current.push(ch);
                    } else {
                        return Err(());
                    }
                }
                ('"', ParsingState::Normal) => {
                    state = ParsingState::String;
                }
                ('"', ParsingState::String) => {
                    state = ParsingState::Normal;
                    let peeked = iter.peek();
                    if peeked.is_none() || peeked != Some(&' ') {
                        return Err(());
                    }
                }
                _ => {
                    current.push(c);
                }
            }
        }

        if let ParsingState::String = state {
            return Err(());
        }

        if current.len() > 0 {
            out.push(current);
        }

        Ok(out)
    }

    fn evaluating_state(
        shell_tx: &Sender<SessionMessage>,
        state: &mut Option<CashState>,
        data: &mut CashShellData,
    ) -> () {
        let args = args_parser(&data.input);
        data.input = String::new();
        let Ok(mut args) = args else {
            shell_tx
                .send(SessionMessage::Terminal(
                    TerminalMessage::PushLine("Error parsing input".into()),
                    None,
                ))
                .unwrap();
            *state = state_transition(shell_tx, *state, data, CashState::Input);
            return;
        };

        if args.len() == 0 {
            *state = state_transition(shell_tx, *state, data, CashState::Input);
            return;
        }

        let (tx, rx) = channel();

        shell_tx
            .send(SessionMessage::Shell(
                ShellMessage::GetCurrentUser,
                Some(tx),
            ))
            .unwrap();

        if let Ok(SessionMessage::Return(ReturnValue::User(Some(user)))) = rx.recv() {
            for root in user.get_path().split(':') {
                let program = args.remove(0);
                let path = Path::new(root).join(&program);

                if let Some(p) =
                    FILESYSTEM
                        .get_by_path(&path)
                        .and_then(|x| match *x.read().unwrap() {
                            FSObject::File {
                                contents: FileType::Program(p),
                                ..
                            } => Some(p),
                            _ => None,
                        })
                {
                    let cwd = data.cwd.clone();
                    let child_shell_tx = shell_tx.clone();

                    let (child_tx, child_rx) = channel();

                    *state = state_transition(shell_tx, *state, data, CashState::Executing);
                    data.running = Some(CashProcess {
                        thread: thread::spawn(move || {
                            p()([vec![cwd], args].concat(), child_rx, child_shell_tx)
                        }),
                        sender: child_tx,
                    });
                    return;
                }

                shell_tx
                    .send(SessionMessage::Terminal(
                        TerminalMessage::PushLine(
                            format!("{}: not an executable file", program).into(),
                        ),
                        None,
                    ))
                    .unwrap();
                *state = state_transition(shell_tx, *state, data, CashState::Input);
                return;
            }
        }

        shell_tx
            .send(SessionMessage::Terminal(
                TerminalMessage::PushLine(format!("{}: command not found", args[0]).into()),
                None,
            ))
            .unwrap();
        *state = state_transition(shell_tx, *state, data, CashState::Input);
    }

    fn executing_state(
        shell_tx: &Sender<SessionMessage>,
        state: &mut Option<CashState>,
        data: &mut CashShellData,
        events: &Receiver<SessionMessage>,
    ) -> () {
        match events.try_recv() {
            Ok(m) => {
                if let SessionMessage::Shell(ShellMessage::ExitCode(_), _) = m {
                    data.running = None;
                    *state = state_transition(&shell_tx, *state, data, CashState::Input);
                    return;
                }
                if let Some(running) = &data.running {
                    if let Err(_) = running.sender.send(m) {
                        data.running = None;
                        *state = state_transition(&shell_tx, *state, data, CashState::Input);
                    }
                }
            }
            Err(TryRecvError::Disconnected) => {
                data.running = None;
                *state = state_transition(&shell_tx, *state, data, CashState::Input);
            }
            _ => {}
        }
    }

    Box::new(move |args, events, shell_tx| {
        let mut state: Option<CashState> = Some(CashState::Initial);
        let mut data = CashShellData {
            cwd: args.first().unwrap().to_string(),
            input: String::new(),
            running: None,
        };

        loop {
            match state {
                Some(CashState::Initial) => {
                    initial_state(&shell_tx, &mut state, &mut data, &events)
                }
                Some(CashState::Input) => input_state(&shell_tx, &mut state, &mut data, &events),
                Some(CashState::Evaluating) => evaluating_state(&shell_tx, &mut state, &mut data),
                Some(CashState::Executing) => {
                    executing_state(&shell_tx, &mut state, &mut data, &events)
                }
                Some(CashState::Interrupting) => break,
                None => {
                    state = state_transition(&shell_tx, state, &data, CashState::Input);
                }
            }
        }
        shell_tx
            .send(SessionMessage::Shell(ShellMessage::ExitCode(0), None))
            .unwrap();
    })
}
