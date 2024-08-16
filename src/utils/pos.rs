use std::ops::AddAssign;

pub struct Point {
    x: usize,
    y: usize,
}

pub struct Size {
    height: usize,
    width: usize,
}

pub struct Rect(Point, Size);

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign for Size {
    fn add_assign(&mut self, rhs: Self) {
        self.height += rhs.height;
        self.width += rhs.width;
    }
}
