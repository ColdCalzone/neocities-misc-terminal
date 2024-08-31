#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use session::{SessionMessage, ShellMessage};
mod session;
pub use session::Session;
pub mod key_events;
mod terminal;
mod utils;

#[cfg(test)]
mod tests;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn greet() {}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn get_session() -> Session {
    session::Session::get_session()
}

pub fn create_input_event(key_event: key_events::KeyEvent) -> SessionMessage {
    SessionMessage::Shell(ShellMessage::InputKeyEvent(key_event), None)
}

pub fn create_interrupt() -> SessionMessage {
    SessionMessage::Interrupt
}
