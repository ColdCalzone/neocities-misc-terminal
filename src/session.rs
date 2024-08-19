use crate::terminal::{
    shell::{user::User, DefaultShell, Shell},
    Terminal,
};

use std::{
    path::PathBuf,
    sync::{
        mpsc::{channel, TryRecvError},
        Mutex,
    },
    thread::JoinHandle,
};
use std::{
    sync::{
        mpsc::{Receiver, Sender},
        Arc,
    },
    thread,
};

#[derive(Debug)]
pub enum InputMessage {
    InputKeyEvent(crate::key_events::KeyEvent),
    ShellChangeCwd(PathBuf),
}

#[derive(Debug)]
pub enum OutputMessage {
    /// Sent to the output handler, should be `terminal.to_string()` 99.99% of the time
    Display(String),
    /// Sent to the `Terminal`
    PushLine(String),
    /// Sent to the `Terminal`, forces it to then send `Display`.
    ForceUpdate,
}

/// A message sent between the input and output threads of the `Session`.
#[derive(Debug)]
pub enum SessionMessage {
    Input(InputMessage),
    Output(OutputMessage),
    Interrupt,
}

/// The highest-level container of a `Terminal` and `Shell`.
/// Largely responsible for async IO.
pub struct Session {
    input: Sender<SessionMessage>,
    output: Sender<SessionMessage>,
    output_handler: Arc<Mutex<Receiver<SessionMessage>>>,
    output_handler_thread: Option<JoinHandle<()>>,
}

impl Session {
    pub fn get_session() -> Self {
        let terminal = Terminal::new();
        let shell = DefaultShell::new_in_home(User::from_name("guest"));

        let (to_output, for_output) = channel::<SessionMessage>();
        let (to_input, for_input) = channel::<SessionMessage>();

        let (output_to_thread, thread_from_output) = channel::<SessionMessage>();

        let _output_thread = thread::spawn(move || {
            let rx = for_output;
            let tx = output_to_thread;
            let mut terminal = terminal;

            loop {
                match rx.recv() {
                    Ok(SessionMessage::Output(OutputMessage::ForceUpdate)) => {
                        tx.send(SessionMessage::Output(OutputMessage::Display(
                            terminal.to_string(),
                        )))
                        .unwrap();
                    }
                    Ok(SessionMessage::Output(OutputMessage::PushLine(x))) => {
                        terminal
                            .get_buffer()
                            .lock()
                            .map(|mut buff| {
                                buff.push(x);
                            })
                            .unwrap();
                    }
                    Err(_) => {
                        eprintln!("Output thread disconnected!");
                        break;
                    }
                    _ => {}
                }
            }
            // tx.send(SessionMessage::Output(OutputMessage::Display(
            //     terminal.to_string(),
            // )))
            // .unwrap();
            // rx.recv().unwrap()
        });

        let tx_output = to_output.clone();
        let _input_thread = thread::spawn(move || {
            let rx = for_input;
            let mut shell = shell;
            loop {
                if shell.get_running_process().is_none() {
                    shell.run_startup(vec![]);
                }
                match rx.try_recv() {
                    Ok(SessionMessage::Input(m)) => {
                        shell.process_message(m);
                    }
                    Ok(SessionMessage::Output(m)) => {
                        tx_output.send(SessionMessage::Output(m)).unwrap();
                    }
                    Err(TryRecvError::Disconnected) => {
                        eprintln!("Input thread disconnected!");
                        break;
                    }
                    _ => {}
                }

                if let Some(process) = shell.get_running_process() {
                    match process.receiver.try_recv() {
                        Ok(SessionMessage::Input(m)) => {
                            shell.process_message(m);
                        }
                        Ok(SessionMessage::Output(m)) => {
                            tx_output.send(SessionMessage::Output(m)).unwrap();
                        }
                        Err(TryRecvError::Disconnected) => {
                            eprintln!("Shell Process: Input thread disconnected!");
                            break;
                        }
                        _ => {}
                    }
                }
            }
        });

        Self {
            input: to_input,
            output: to_output,
            output_handler: Arc::new(Mutex::new(thread_from_output)),
            output_handler_thread: None,
        }
    }

    /// *Pending a rename.* \
    /// Accepts a closure which takes `SessionMessage`s as an argument
    pub fn output_handler(&mut self, output_function: fn(String) -> ()) {
        let rx = self.output_handler.clone();
        self.output_handler_thread = Some(thread::spawn(move || {
            let lock = match rx.lock() {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("Error locking output receiver: {e}");
                    return;
                }
            };
            loop {
                let message = match lock.recv() {
                    Ok(x) => x,
                    Err(e) => {
                        println!("Message failed on output handler, probably no big deal, but...");
                        eprintln!("Error: {e}");
                        break;
                    }
                };

                match message {
                    SessionMessage::Output(x) => {
                        if let OutputMessage::Display(x) = x {
                            output_function(x);
                        }
                    }
                    SessionMessage::Interrupt => {
                        println!("Output Handler: Interrupt received!");
                        break;
                    }
                    _ => {
                        println!("Output: Received unsupported message {:?}", message);
                    }
                }
            }
            println!("Output thread closed");
        }));
    }
}
