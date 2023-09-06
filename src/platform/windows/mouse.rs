use winapi::{
    shared::windef::POINT,

    um::winuser::{GetCursorPos, SetCursorPos, mouse_event, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP},
};
use super::types::Point;

#[derive(Debug, Clone, Copy)]
pub struct Mouse {
    pub position: Point,
}

impl Mouse {
    pub fn new() -> Self {
        let position = Self::get_position();
        Self { position }
    }

    pub fn get_position() -> Point {
        let mut point = POINT { x: 0, y: 0 };
        unsafe {
            GetCursorPos(&mut point);
        }
        Point::new(point.x, point.y)
    }

    pub fn set_position(&mut self, position: Point) {
        self.position = position;
        unsafe {
            SetCursorPos(position.x, position.y);
        }
    }

    pub fn click_down() {
        unsafe {
            mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
        }
    }

    pub fn click_up() {
        unsafe {
            mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
        }
    }
}
