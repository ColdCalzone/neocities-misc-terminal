// CASH -- Cold's Awful SHell
// Name derived from CA$H, the R.A.M Demo's (second) hardest challenge.
fn run() -> Box<dyn FnOnce(Vec<String>, Receiver<SessionMessage>, Sender<SessionMessage>)> {
    use crate::key_events::*;
    use crate::session::{SessionMessage, ShellMessage};
    let mut input: String = String::new();

    fn get_prefix(shell_tx: &mut Sender<SessionMessage>) -> String {
        //     let path = match self.cwd.lock() {
        //         Ok(x) => x,
        //         Err(e) => panic!("Error getting input prefix: {e}"),
        //     };
        //     format!(
        //         "[{}@deep-freezer:{}]$",
        //         self.user.get_name(),
        //         path.to_string_lossy()
        // )
        String::from("$")
    }

    Box::new(move |args, events, shell_tx| loop {
        match events.recv() {
            Ok(SessionMessage::Shell(ShellMessage::InputKeyEvent(key_event), _)) => {
                match key_event.key_type {
                    Key::Char(data) => {
                        if let Some(Modifier::Shift) = key_event.modifier {
                            input.push(data.to_ascii_uppercase());
                        } else {
                            input.push(data.to_ascii_lowercase());
                        }
                    }
                    Key::Backspace => {
                        input.pop();
                    }
                    _ => {}
                }
            }
            Ok(SessionMessage::Interrupt) => break,
            _ => {}
        }
    })
}
