use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::DataExchange::{AddClipboardFormatListener, CloseClipboard, GetClipboardData},
    Win32::System::Memory::{GlobalLock, GlobalUnlock},
    Win32::System::{DataExchange::OpenClipboard, LibraryLoader::GetModuleHandleA},
    Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    unsafe {
        // Get module handle to calling process by passing None
        let module_instance = GetModuleHandleA(None)?;
        debug_assert!(module_instance.0 != 0);

        // Register window class
        let window_class = s!("clippers-window");
        let wc = WNDCLASSA {
            hInstance: module_instance.into(),
            lpszClassName: window_class,
            // Use wndproc function below
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        // Register window class and save the returned class ID
        let atom = RegisterClassA(&wc);
        println!("Just for fun, atom is {atom}"); // TODO: delete
        debug_assert!(atom != 0);

        // Create window
        let window_handle = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("clippers-main"),
            WS_DISABLED,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            module_instance,
            None,
        );

        // Add this window as a clipboard format listener
        AddClipboardFormatListener(window_handle)?;

        // Listener loop
        let mut message = MSG::default();
        while GetMessageA(&mut message, None, 0, 0).into() {
            DispatchMessageA(&message);
        }

        Ok(())
    }
}

// Process messages sent to this window
extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_CREATE => {
                println!("Window created!");
                LRESULT(0)
            }
            WM_CLIPBOARDUPDATE => {
                println!("CLIPBOARD UPDATED!!!");
                // Get lock on clipboard
                while let Err(_) = OpenClipboard(window) {
                    println!("Failed to open clipboard");
                }
                println!("Successfully opened clipboard");

                // Get handle clipboard data of type text
                // TODO: can't convert CLIPBOARD_FORMAT to u16 (to then go to u32 which GetClipboardData takes), fudge it for now
                const CF_TEXT: u16 = 1u16;
                let cb_data_handle = if let Ok(h) = GetClipboardData(CF_TEXT as u32) {
                    println!("Got handle to clipboard data");
                    h
                } else {
                    println!("Clipboard data was not CF_TEXT or handle fetch failed");
                    // TODO: use closure to do this so that we can unconditionally CloseClipboard with only one line
                    while let Err(_) = CloseClipboard() {
                        println!("Failed to close clipboard. Retrying...");
                    }
                    return LRESULT(1); // TODO error codes?
                };

                // Use handle to get clipboard data via a GlobalLock
                let cb_data_handle: HGLOBAL = core::mem::transmute(cb_data_handle);
                let data = loop {
                    let data = GlobalLock(cb_data_handle);
                    if !data.is_null() {
                        break data;
                    }
                    println!("GlobalLock returned null. Retrying...");
                };
                let data = std::ffi::CStr::from_ptr(data as *const _).to_string_lossy();
                println!("You copied: \"{data}\"");

                // Close resources and return
                while let Err(_) = GlobalUnlock(cb_data_handle) {
                    println!("Failed to GlobalUnlock. Retrying...");
                }
                while let Err(_) = CloseClipboard() {
                    println!("Failed to close clipboard. Retrying...");
                }
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
