use std::ops::AddAssign;

#[derive(Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Clone, Copy)]
pub struct Size {
    height: usize,
    width: usize,
}

impl AddAssign for Size {
    fn add_assign(&mut self, rhs: Self) {
        self.height += rhs.height;
        self.width += rhs.width;
    }
}

impl Into<Rect> for Size {
    fn into(self) -> Rect {
        Rect {
            size: self,
            origin: Point::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Rect {
    origin: Point,
    size: Size,
}

impl Rect {
    pub fn point_within(&self, point: &Point) -> bool {
        point.x > self.origin.x
            && point.x < (self.origin.x + self.size.width)
            && point.y > self.origin.y
            && point.y < (self.origin.y + self.size.height)
    }

    pub fn set_origin(&mut self, point: Point) {
        self.origin = point;
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn with_origin(self, point: Point) -> Self {
        Self {
            origin: point,
            ..self
        }
    }

    pub fn with_size(self, size: Size) -> Self {
        Self { size, ..self }
    }
}
