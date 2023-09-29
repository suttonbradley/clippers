use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::DataExchange::{
    AddClipboardFormatListener, CloseClipboard, GetClipboardData, OpenClipboard,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::Memory::{GlobalLock, GlobalUnlock};
use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, MOD_WIN};
use windows::Win32::UI::WindowsAndMessaging::*;

// Code for text-type clipboard entries
const CF_TEXT: u16 = 1u16;
// Virtual keycode for the 'V' key
const VK_LETTER_V: u32 = 0x56;
// Hotkey ID
const HOTKEY_ID_WIN_V: i32 = 0xBEEF;

pub(crate) fn run_loop() {
    unsafe {
        // Get module handle to calling process by passing None
        // TODO: pop-up if things like this fail
        let module_instance = GetModuleHandleA(None).expect("Failed to get module handle");
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

        // Register Win+V
        while let Err(_) = RegisterHotKey(window_handle, HOTKEY_ID_WIN_V, MOD_WIN, VK_LETTER_V) {
            println!("Failed to register hotkey");
        }
        // TODO: call UnregisterHotKey?

        // Add this window as a clipboard format listener
        while let Err(_) = AddClipboardFormatListener(window_handle) {
            println!("Failed to add ClipboardFormatListener");
        }

        // Listener loop
        let mut message = MSG::default();
        while GetMessageA(&mut message, None, 0, 0).into() {
            DispatchMessageA(&message);
        }
    }
}

// Process messages sent to this window
extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_CLIPBOARDUPDATE => {
                // Get lock on clipboard
                while let Err(_) = OpenClipboard(window) {
                    println!("Failed to open clipboard");
                }

                // Get handle clipboard data of type text
                // TODO: can't convert CLIPBOARD_FORMAT to u16 (to then go to u32 which GetClipboardData takes), fudge it for now
                let cb_data_handle = if let Ok(h) = GetClipboardData(CF_TEXT as u32) {
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
                println!("COPIED: \"{data}\"");

                // Close resources and return
                while let Err(_) = GlobalUnlock(cb_data_handle) {
                    println!("Failed to GlobalUnlock. Retrying...");
                }
                while let Err(_) = CloseClipboard() {
                    println!("Failed to close clipboard. Retrying...");
                }

                LRESULT(0)
            }
            WM_HOTKEY => {
                let hotkey_id: isize = std::mem::transmute(wparam);
                let hotkey_id: i32 = i32::try_from(hotkey_id)
                    .expect("Failed to cast hotkey_id from isize down to i32");
                debug_assert!(hotkey_id == HOTKEY_ID_WIN_V);

                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
