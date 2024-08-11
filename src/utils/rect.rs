use std::ops::AddAssign;

pub struct Rectangle {
    height: usize,
    width: usize,
}

impl AddAssign for Rectangle {
    fn add_assign(&mut self, rhs: Self) {
        self.height += rhs.height;
        self.width += rhs.width;
    }
}
