use crate::{
    key_events::*,
    terminal::{
        shell::{
            user::{SignInError, User},
            DefaultShell, Shell,
        },
        DefaultTerminal, Span, Terminal,
    },
};

use std::{path::PathBuf, sync::mpsc::channel};
use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
};

pub enum EventLoopError {
    ChannelClosed,
}

pub enum BudgetNever {}

pub trait EventLoop {
    fn event_loop(
        &mut self,
        receiver: Receiver<SessionMessage>,
        sender: Sender<SessionMessage>,
    ) -> Result<BudgetNever, EventLoopError>;
}

/// Messages sent to the shell / running processes
#[derive(Debug)]
pub enum ShellMessage {
    /// Sent to the shell to pass to the current running program
    InputKeyEvent(KeyEvent),
    /// Sent to the shell to pass to the current running program
    ExitCode(u32),
    /// Sent to the shell to instruct it to change the CWD
    ChangeCwd(PathBuf),
    /// Sent to the shell, asking it to return the current user
    GetCurrentUser,
    /// Sent to the shell, asking it to sign in to the given user with the given password hash
    /// Returns a Result<(), SignInError>
    TrySetUser(String, Option<u64>),
}

/// Messages sent to the terminal
#[derive(Debug)]
pub enum TerminalMessage {
    /// Sent to the terminal to add the line to the buffer
    PushLine(String),
    /// Sent to the terminal to add the text to the current line of the buffer
    Push(String),
    /// Sent to the terminal to add the text to the current line of the buffer
    PushSpan(Span),
    /// Clear the screen
    Clear,
    /// Clear the specified span. Counts from the bottom.
    ClearSpan(usize),
    /// Set the specified span. Counts from the bottom.
    SetSpan(usize, Span),
    /// Delete the specified span. Counts from the bottom.
    DeleteSpan(usize),
    /// Send to the terminal to make it send OutputMessage::Display with its contents
    ForceUpdate,
}

/// Messages sent to the output handler
#[derive(Debug)]
pub enum OutputMessage {
    /// Display payload to the screen. Should be `terminal.to_string()` 99.99% of the time
    Display(String),
    /// To be handled by the Session host, i.e. open something.
    Signal,
}

#[derive(Debug)]
pub enum ReturnValue {
    User(Option<User>),
    SignInResult(Result<(), SignInError>),
}

/// A message sent between the input and output threads of the `Session`.
#[derive(Debug)]
pub enum SessionMessage {
    Terminal(TerminalMessage, Option<Sender<SessionMessage>>),
    Shell(ShellMessage, Option<Sender<SessionMessage>>),
    Output(OutputMessage, Option<Sender<SessionMessage>>),
    Interrupt,
    Resize(usize, usize),
    Ack(Option<Sender<SessionMessage>>),
    Return(ReturnValue),
    KillSessionYesReallyTheActualSessionNotSomeInternalThing,
}

/// The highest-level container of a `Terminal` and `Shell`.
/// Largely responsible for async IO.
pub struct Session {
    input: Sender<SessionMessage>,
    output: Sender<SessionMessage>,
    receiver: Receiver<SessionMessage>,
    sender_self: Sender<SessionMessage>,
    output_handler: Option<Sender<SessionMessage>>,
    input_handler: Option<Sender<SessionMessage>>,
}

impl Session {
    pub fn get_session() -> Self {
        let mut term = DefaultTerminal::new();
        let mut shell = DefaultShell::new_in_home(User::from_name("guest"));

        let (tx, rx) = channel::<SessionMessage>();

        let tx_shell = tx.clone();
        let tx_term = tx.clone();

        let (tx_to_input, rx_to_input) = channel::<SessionMessage>();
        let (tx_to_output, rx_to_output) = channel::<SessionMessage>();

        let _output_thread = thread::spawn(move || {
            term.event_loop(rx_to_output, tx_term);
        });

        let _input_thread = thread::spawn(move || {
            shell.event_loop(rx_to_input, tx_shell);
        });

        Self {
            input: tx_to_input,
            output: tx_to_output,
            receiver: rx,
            sender_self: tx,
            output_handler: None,
            input_handler: None,
        }
    }

    /// *Pending a rename.* \
    /// Accepts a closure which takes `SessionMessage`s as an argument
    pub fn output_handler(&mut self, output_function: fn(String) -> ()) {
        let (tx, rx) = channel::<SessionMessage>();
        self.output_handler = Some(tx);
        thread::spawn(move || {
            loop {
                let message = match rx.recv() {
                    Ok(x) => x,
                    Err(e) => {
                        println!("Message failed on output handler, probably no big deal, but...");
                        eprintln!("Error: {e}");
                        break;
                    }
                };

                match message {
                    SessionMessage::Output(x, _) => {
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
        });
    }

    pub fn input_handler(&mut self, input_function: fn() -> Option<SessionMessage>) {
        let (tx, rx) = channel::<SessionMessage>();
        let sender = self.sender_self.clone();
        self.input_handler = Some(tx);
        thread::spawn(move || loop {
            if let Some(m) = input_function() {
                sender.send(m).unwrap();
            }
        });
    }

    pub fn run(self) {
        loop {
            let message_result = self.receiver.recv();
            match message_result {
                Ok(m) => match m {
                    SessionMessage::Shell(_, _) => {
                        self.input.send(m).unwrap();
                    }
                    SessionMessage::Terminal(_, _) => {
                        self.output.send(m).unwrap();
                    }
                    SessionMessage::Output(_, _) => {
                        if let Some(output_handler) = &self.output_handler {
                            output_handler.send(m).unwrap();
                        }
                    }
                    SessionMessage::Resize(_, _) => todo!(),
                    SessionMessage::Interrupt => {
                        self.input.send(SessionMessage::Interrupt).unwrap();
                    }
                    SessionMessage::Ack(..) => {}
                    SessionMessage::Return(..) => {}
                    SessionMessage::KillSessionYesReallyTheActualSessionNotSomeInternalThing => {
                        break;
                    }
                },
                Err(e) => {
                    eprintln!("Session has encountered an error: {e}");
                    break;
                }
            }
        }
        println!("Session closed");
    }
}
