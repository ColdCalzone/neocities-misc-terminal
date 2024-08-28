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
pub use style::{Color, Span, SpanSet};

pub trait Terminal<B>: EventLoop + fmt::Display {
    fn new() -> Self;

    fn set_size(&mut self, rect: Size);
    fn get_buffer(&mut self) -> Arc<Mutex<B>>;
    fn clear(&self);
}

pub struct DefaultTerminal {
    buffer: Arc<Mutex<SpanSet>>,
    scroll: usize,
    size: Option<Size>,
}

impl Terminal<SpanSet> for DefaultTerminal {
    fn new() -> Self {
        let mut span_set = SpanSet::new();
        let mut span = Span::new();
        span.fg_color = Some(Color::new_rgb(255, 255, 0));
        // span.bg_color = Some(Color::new_rgb(0, 0, 0));
        span_set.push(span);
        Self {
            buffer: Arc::new(Mutex::new(span_set)),
            scroll: 0,
            size: None,
        }
    }

    fn set_size(&mut self, rect: Size) {
        self.size = Some(rect);
    }

    fn get_buffer(&mut self) -> Arc<Mutex<SpanSet>> {
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
                Ok(SessionMessage::Terminal(msg, sender)) => match msg {
                    TerminalMessage::ForceUpdate => {
                        tx.send(SessionMessage::Output(
                            OutputMessage::Display(self.to_string()),
                            None,
                        ))
                        .unwrap();
                    }
                    TerminalMessage::PushLine(x) => {
                        self.get_buffer()
                            .lock()
                            .map(|mut buff| {
                                if let Some(span) = buff.last_mut() {
                                    span.text.push_str(&x);
                                    span.text.push_str("\r\n");
                                }
                            })
                            .unwrap();
                    }
                    TerminalMessage::Push(x) => {
                        self.get_buffer()
                            .lock()
                            .map(|mut buff| {
                                if let Some(span) = buff.last_mut() {
                                    span.text.push_str(&x);
                                }
                            })
                            .unwrap();
                    }
                    TerminalMessage::PushSpan(x) => {
                        self.get_buffer()
                            .lock()
                            .map(|mut buff| {
                                buff.push(x);
                            })
                            .unwrap();
                    }
                    TerminalMessage::Clear => {
                        self.clear();
                    }
                    TerminalMessage::ClearSpan(index) => {
                        self.get_buffer()
                            .lock()
                            .map(|mut buff| {
                                let len = buff.len();
                                if let Some(span) = buff.get_mut(len - index - 1) {
                                    span.text = "".into();
                                }
                            })
                            .unwrap();
                    }
                    TerminalMessage::SetSpan(index, new_span) => {
                        self.get_buffer()
                            .lock()
                            .map(|mut buff| {
                                let len = buff.len();
                                let index = len - index - 1;
                                if let Some(span) = buff.get_mut(index) {
                                    *span = new_span;
                                }
                            })
                            .unwrap();
                    }
                    TerminalMessage::DeleteSpan(index) => {
                        self.get_buffer()
                            .lock()
                            .map(|mut buff| {
                                let len = buff.len();
                                buff.remove(len - index - 1);
                            })
                            .unwrap();
                    }
                },
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
