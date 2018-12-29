pub struct Rect2d {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rect2d {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Rect2d {
        Rect2d {
            x,
            y,
            width,
            height,
        }
    }
}
