extern crate misc_terminal;
use std::io;

use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute, style, terminal,
};

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;

    execute!(std::io::stdout(), terminal::EnterAlternateScreen,)?;

    let mut session = misc_terminal::get_session();
    session.output_handler(|display| {
        println!("{display}");
    });

    loop {
        match event::read()? {
            event::Event::Key(key_event) => {
                println!("{key_event:?}");
                if key_event.code == KeyCode::Char('c')
                    && key_event.modifiers == KeyModifiers::CONTROL
                {
                    break;
                }
            }
            event::Event::Paste(x) => {}
            _ => {}
        }
    }

    execute!(std::io::stdout(), terminal::LeaveAlternateScreen,)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
