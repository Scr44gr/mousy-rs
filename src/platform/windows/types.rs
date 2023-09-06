use winapi::shared::windef::HWND;


#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Window {
    pub hwnd: HWND,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Window {
    pub fn new(hwnd: HWND) -> Self {
        Self { hwnd }
    }
}