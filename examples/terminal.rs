extern crate misc_terminal;
use std::{io, sync::mpsc::channel};

use crossterm::{
    cursor,
    event::{self, KeyCode, KeyModifiers},
    execute, terminal,
};

use misc_terminal::{create_input_event, create_interrupt, key_events};

fn main() -> io::Result<()> {
    // terminal::enable_raw_mode()?;

    execute!(std::io::stdout(), terminal::EnterAlternateScreen,)?;

    let mut session = misc_terminal::get_session();
    session.output_handler(|display| {
        execute!(
            std::io::stdout(),
            cursor::MoveTo(0, 0),
            terminal::Clear(terminal::ClearType::All),
        )
        .unwrap();
        print!("{display}");
    });

    session.input_handler(|| {
        if let Ok(event) = event::read() {
            match event {
                event::Event::Key(key_event) => {
                    if key_event.code == KeyCode::Char('c')
                        && key_event.modifiers == KeyModifiers::CONTROL
                    {
                        return Some(create_interrupt());
                    }

                    let key_event_internal = key_events::KeyEvent {
                        key_type: match key_event.code {
                            KeyCode::Backspace => key_events::Key::Backspace,
                            KeyCode::Enter => key_events::Key::Enter,
                            KeyCode::Char(x) => key_events::Key::Char(x.to_ascii_lowercase()),
                            _ => return None,
                        },
                        modifier: match key_event.modifiers {
                            KeyModifiers::SHIFT => Some(key_events::Modifier::Shift),
                            KeyModifiers::CONTROL => Some(key_events::Modifier::Ctrl),
                            KeyModifiers::ALT => Some(key_events::Modifier::Alt),
                            _ => None,
                        },
                    };

                    return Some(create_input_event(key_event_internal));
                }
                event::Event::Paste(x) => {}
                _ => {}
            }
        }
        None
    });

    session.run();

    execute!(std::io::stdout(), terminal::LeaveAlternateScreen,)?;
    // terminal::disable_raw_mode()?;

    Ok(())
}
