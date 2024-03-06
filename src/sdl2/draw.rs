use sdl2::rect::Rect;

use crate::draw::DrawRect;

impl DrawRect {
    pub fn to_rect(&self) -> Rect {
        Rect::new(
            self.x as i32,
            self.y as i32,
            self.width as u32,
            self.height as u32,
        )
    }
}
