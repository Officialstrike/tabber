fn main() {
    windows::build!(
        Windows::Win32::UI::WindowsAndMessaging::{
            FindWindowExW, GetWindowTextW, GetWindowPlacement, ShowWindow,
            HWND, WINDOWPLACEMENT,
            SW_SHOWMINIMIZED, SW_SHOWMAXIMIZED, SW_SHOWNORMAL ,SW_MINIMIZE, SW_MAXIMIZE, SW_RESTORE
        },
        Windows::Win32::System::SystemServices::{PWSTR},

    );
}
