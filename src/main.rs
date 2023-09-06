pub mod platform {
    #[cfg(target_os = "windows")]
    pub mod windows {
        pub mod mouse;
        pub mod types;
        pub mod keyboard;
        pub mod keys;
    }
}

#[cfg(target_os = "windows")]
use platform::windows::{mouse, keys, keyboard};

const SENSIBILITY: f32 = 200.0;

fn main() {
    let mut mouse = mouse::Mouse::new();
    let keyboard = keyboard::Keyboard::new();
    let mut delta_time = 0;

    loop {
        let time_now = std::time::Instant::now();
        let mouse_speed = (SENSIBILITY * delta_time as f32 / 1000.0) as i32;
        let mut position = mouse::Mouse::get_position();

        if keyboard.is_both_keys_pressed(keys::Keys::Shift, keys::Keys::Left) {
            position.x -= mouse_speed;
            mouse.set_position(position);
        }
        if keyboard.is_both_keys_pressed(keys::Keys::Shift, keys::Keys::Right) {
            position.x += mouse_speed;
            mouse.set_position(position);
        }
        if keyboard.is_both_keys_pressed(keys::Keys::Shift, keys::Keys::Up) {
            position.y -= mouse_speed;
            mouse.set_position(position);
        }
        if keyboard.is_both_keys_pressed(keys::Keys::Shift, keys::Keys::Down) {
            position.y += mouse_speed;
            mouse.set_position(position);
        }

        if keyboard.is_both_keys_pressed(keys::Keys::Shift, keys::Keys::Esc) {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(30));
        delta_time = time_now.elapsed().as_millis();
    }

}
