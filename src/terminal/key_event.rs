enum Modifier {
    Shift = 0b001,
    Ctrl = 0b010,
    Alt = 0b100,
}

enum Key {
    Char { data: char },
    Mod,
    Backspace,
    Enter,
}

pub struct KeyEvent {
    key_type: Key,
    modifier: Modifier,
}
