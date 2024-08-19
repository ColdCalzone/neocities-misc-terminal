pub mod files;
pub mod user;
use files::{FSObject, FileSystem, FileType, FILESYSTEM};
use std::{
    path::{Path, PathBuf},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
    thread::{self, JoinHandle},
};
use user::User;

use crate::{
    key_events::KeyEvent,
    session::{InputMessage, SessionMessage},
};

pub trait Shell {
    fn new_at_path(path: PathBuf) -> Self;

    fn with_user(self, user: User) -> Self;

    fn new_in_home(user: User) -> Self;

    fn process_message(&mut self, message: InputMessage);

    fn get_running_process(&self) -> &Option<RunningProcess>;

    fn run_program(&mut self, program: fn() -> files::Program, args: Vec<String>);

    fn run_startup(&mut self, args: Vec<String>);
}

pub struct RunningProcess {
    pub thread: JoinHandle<()>,
    pub sender: Sender<SessionMessage>,
    pub receiver: Receiver<SessionMessage>,
}

pub struct DefaultShell {
    cwd: Mutex<PathBuf>,
    user: Option<User>,
    running: Option<RunningProcess>,
    startup: fn() -> files::Program,
}

impl Shell for DefaultShell {
    fn new_at_path(path: PathBuf) -> Self {
        Self {
            cwd: Mutex::new(path),
            user: None,
            running: None,
            startup: {
                let file_arc = (*FILESYSTEM)
                    .get_by_path(&PathBuf::from("/bin/help"))
                    .unwrap();
                let file = file_arc.read().unwrap();

                let cash = match *file {
                    FSObject::File {
                        contents: FileType::Program(x),
                        ..
                    } => x,
                    _ => unreachable!(),
                };

                cash
            },
        }
    }

    fn with_user(mut self, user: User) -> Self {
        self.user = Some(user);
        self
    }

    fn new_in_home(user: User) -> Self {
        Self::new_at_path(format!("/home/{}", user.get_name()).into()).with_user(user)
    }

    fn process_message(&mut self, message: InputMessage) {
        match message {
            InputMessage::ShellChangeCwd(path) => {
                *(self.cwd.lock()).unwrap() = path.clone();
                return;
            }
            _ => {}
        }
        if let Some(ref running) = self.running {
            running.sender.send(SessionMessage::Input(message));
        }
    }

    fn get_running_process(&self) -> &Option<RunningProcess> {
        &self.running
    }

    fn run_program(&mut self, program: fn() -> files::Program, args: Vec<String>) {
        let (tx_ev, rx_ev) = channel::<SessionMessage>();
        let (tx_sh, rx_sh) = channel::<SessionMessage>();
        self.running = Some(RunningProcess {
            thread: thread::spawn(move || program()(args, rx_ev, tx_sh)),
            sender: tx_ev,
            receiver: rx_sh,
        })
    }

    fn run_startup(&mut self, args: Vec<String>) {
        self.run_program(self.startup, args);
    }
}
