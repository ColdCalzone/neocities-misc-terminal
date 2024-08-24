use crate::{session::*, utils::pos::Size};
use std::{
    fmt,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
};

pub mod shell;
pub mod style;

pub trait Terminal<B>: EventLoop + fmt::Display {
    fn new() -> Self;

    fn set_size(&mut self, rect: Size);
    fn get_buffer(&mut self) -> Arc<Mutex<B>>;
    fn clear(&self);
}

pub struct DefaultTerminal {
    buffer: Arc<Mutex<String>>,
    scroll: usize,
    size: Option<Size>,
}

impl Terminal<String> for DefaultTerminal {
    fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(String::new())),
            scroll: 0,
            size: None,
        }
    }

    fn set_size(&mut self, rect: Size) {
        self.size = Some(rect);
    }

    fn get_buffer(&mut self) -> Arc<Mutex<String>> {
        self.buffer.clone()
    }

    fn clear(&self) {
        let lock = (*self.buffer).lock();
        match lock {
            Ok(mut x) => {
                x.clear();
            }
            Err(e) => {
                panic!("terminal clear: Couldn't get write access to buffer: {e}");
            }
        }
    }
}

impl EventLoop for DefaultTerminal {
    fn event_loop(
        &mut self,
        rx: Receiver<SessionMessage>,
        tx: Sender<SessionMessage>,
    ) -> Result<BudgetNever, EventLoopError> {
        loop {
            match rx.recv() {
                Ok(SessionMessage::Terminal(TerminalMessage::ForceUpdate, _)) => {
                    tx.send(SessionMessage::Output(
                        OutputMessage::Display(self.to_string()),
                        None,
                    ))
                    .unwrap();
                }
                Ok(SessionMessage::Terminal(TerminalMessage::PushLine(x), _)) => {
                    self.get_buffer()
                        .lock()
                        .map(|mut buff| {
                            buff.push_str(&x);
                            buff.push('\n');
                        })
                        .unwrap();
                }
                Ok(SessionMessage::Terminal(TerminalMessage::Push(x), _)) => {
                    self.get_buffer()
                        .lock()
                        .map(|mut buff| {
                            buff.push_str(&x);
                        })
                        .unwrap();
                }
                Err(_) => {
                    eprintln!("Output thread disconnected!");
                    return Err(EventLoopError::ChannelClosed);
                }
                _ => {}
            }
        }
    }
}

impl fmt::Display for DefaultTerminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lock = self.buffer.lock();
        match lock {
            Ok(x) => {
                write!(f, "{x}")
            }
            Err(e) => panic!("Displaying terminal: Couldn't lock buffer: {e}"),
        }
    }
}
