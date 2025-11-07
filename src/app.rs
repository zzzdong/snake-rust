pub enum State {
    Begin,
    Playing,
    Paused,
    Exit,
    End(End),
}

pub enum End {
    Win,
    Lost,
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    w: i32,
    h: i32,
}
impl Size {
    pub fn new(w: i32, h: i32) -> Size {
        Size { w, h }
    }

    pub fn width(&self) -> i32 {
        self.w
    }

    pub fn height(&self) -> i32 {
        self.h
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn is_same(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}
