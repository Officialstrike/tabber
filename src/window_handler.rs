pub mod window {
    use bindings::Windows::Win32::{System::SystemServices::PWSTR, UI::WindowsAndMessaging};
    use regex::Regex;

    /// Searches for a string that ends in "Visual Studio Code" using regex, it returns a handle to the window if found.
    /// # Example
    ///
    /// ```
    ///if let Ok(window) = window::find() {
    /// // Check if the window is found, if so, do something with the handle here.
    ///}
    /// ```
    pub fn find() -> Result<WindowsAndMessaging::HWND, &'static str> {
        let mut window_handle = None;
        let mut window_name = [0; 512];

        // Loop's until the FindWindowExW func runs out of windows or if the correct window is found.
        loop {
            // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindowexw
            window_handle = Some(unsafe {
                WindowsAndMessaging::FindWindowExW(None, window_handle, "Chrome_WidgetWin_1", None)
            });

            // if FindWindowExW retuns Null(0) as a handle, we return an Error.
            if window_handle.unwrap().0 == 0 {
                return Err("Window not found!");
            }

            /*
            Return the title of the window, and store it in window_name.
            GetWindowTextW returns the lenght of the title, and we store it in window_len.
            https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextw
            */
            let window_len = unsafe {
                WindowsAndMessaging::GetWindowTextW(
                    window_handle,
                    PWSTR(window_name.as_mut_ptr()),
                    window_name.len() as i32,
                )
            };

            // convert window_name to string we can use to compare our string to.
            let window_name = String::from_utf16_lossy(&window_name[..window_len as usize]);

            // If window_name ends with Visual Studio Code, return the window handle.
            if Regex::new(r"Visual Studio Code$")
                .unwrap()
                .is_match(&window_name)
            {
                return Ok(window_handle.unwrap());
            }
        }
    }

    /// If window is minimized -> restore to the orginal size. <br>
    /// if window is visible -> minimize.
    /// # Example
    ///
    /// ```
    /// window::toggle_show_state(window_handle); // open/minimize the window.
    /// ```
    pub fn toggle_show_state(window_handle: WindowsAndMessaging::HWND) {
        // Variable that contains the info about the placement of a window.
        let mut window_info = WindowsAndMessaging::WINDOWPLACEMENT::default();

        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowplacement
        unsafe { WindowsAndMessaging::GetWindowPlacement(window_handle, &mut window_info) };

        // if the window state is minimized -> maximized | resized(restore).
        // if the window is maximized | resized -> minimize.
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
        match window_info.showCmd {
            WindowsAndMessaging::SW_SHOWMINIMIZED => unsafe {
                WindowsAndMessaging::ShowWindow(window_handle, WindowsAndMessaging::SW_RESTORE);
            },

            WindowsAndMessaging::SW_SHOWMAXIMIZED | WindowsAndMessaging::SW_SHOWNORMAL => unsafe {
                WindowsAndMessaging::ShowWindow(window_handle, WindowsAndMessaging::SW_MINIMIZE);
            },

            _ => {}
        }
    }
}
