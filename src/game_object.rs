use tcod::Color;

pub struct GameObject {
    pub x: i32,
    pub y: i32,
    pub char: char,
    pub color: Color
}

impl GameObject {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        GameObject { x, y, char, color }
    }
}