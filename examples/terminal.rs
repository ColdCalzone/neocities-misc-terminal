extern crate misc_terminal;
use std::io;

use crossterm::{execute, style, terminal};

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;

    execute!(
        std::io::stdout(),
        terminal::Clear(terminal::ClearType::All),
        terminal::Clear(terminal::ClearType::Purge),
    )?;
    execute!(std::io::stdout(), style::Print("Hello, world!"))?;

    terminal::disable_raw_mode()?;

    Ok(())
}
