#[derive(Debug, PartialEq, Eq)]
pub enum Modifier {
    Shift = 0b001,
    Ctrl = 0b010,
    Alt = 0b100,
}

#[derive(Debug)]
pub enum Key {
    Char { data: char },
    Mod,
    Backspace,
    Enter,
}

#[derive(Debug)]
pub struct KeyEvent {
    pub key_type: Key,
    pub modifier: Modifier,
}
