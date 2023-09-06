use winapi::um::winuser::{GetForegroundWindow, GetWindowTextW, GetAsyncKeyState, SetFocus, BlockInput, GetDesktopWindow};
use super::{types::Window, keys::Keys};


#[derive(Debug, Clone, Copy)]
pub struct Keyboard {
    pub window: Window,
}

impl Keyboard {
    pub fn new() -> Self {
        let window = Self::get_window();
        Self { window }
    }

    pub fn get_window() -> Window {
        let hwnd = unsafe { GetForegroundWindow() };
        Window::new(hwnd)
    }

    pub fn get_window_title(&self) -> String {
        let mut buffer = [0u16; 256];
        unsafe {
            GetWindowTextW(self.window.hwnd, buffer.as_mut_ptr(), buffer.len() as i32);
        }
        String::from_utf16_lossy(&buffer)
    }

    pub fn is_key_pressed(&self, key: i32) -> bool {
        let state = unsafe { GetAsyncKeyState(key) } as u16;
        state & 0x8000 != 0
    }

    pub fn is_key_toggled(&self, key: i32) -> bool {
        let state = unsafe { GetAsyncKeyState(key) } as u16;
        state & 0x0001 != 0
    }

    pub fn is_both_keys_pressed(&self, key1: Keys, key2: Keys) -> bool {
        let state1 = unsafe { GetAsyncKeyState(key1 as i32) } as u16;
        let state2 = unsafe { GetAsyncKeyState(key2 as i32) } as u16;
        state1 & 0x8000 != 0 && state2 & 0x8000 != 0
    }
    pub fn is_both_keys_release(&self, key1: Keys, key2: Keys) -> bool {
        let state1 = unsafe { GetAsyncKeyState(key1 as i32) } as u16;
        let state2 = unsafe { GetAsyncKeyState(key2 as i32) } as u16;
        state1 & 0x8000 == 0 && state2 & 0x8000 == 0
    }

    pub fn focus_desktop(&self) {
        unsafe {
            SetFocus(GetDesktopWindow());
        }
    }

    pub fn block_input(&self) {
        unsafe {println!("BlockInput: {}", BlockInput(1));}
    }

    pub fn unblock_input(&self) {
        unsafe {println!("BlockInput: {}", BlockInput(0));}
    }
}