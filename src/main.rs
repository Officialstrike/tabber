#![windows_subsystem = "windows"]
mod window_handler;
use window_handler::window;

fn main() {
    // Call window find, if ok toggle_show_state of the handle of the window.
    if let Ok(window_handle) = window::find() {
        window::toggle_show_state(window_handle);
    }
}
