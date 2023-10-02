use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::DataExchange::{
    AddClipboardFormatListener, CloseClipboard, GetClipboardData, OpenClipboard,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::Memory::{GlobalLock, GlobalUnlock};
use windows::Win32::System::Ole::{CF_TEXT, CLIPBOARD_FORMAT};
use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, MOD_WIN, VIRTUAL_KEY, VK_V};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::util::clip_store_op;

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
        // TODO: add retries/backoff and fail after some time
        while let Err(_) = RegisterHotKey(
            window_handle,
            HOTKEY_ID_WIN_V,
            MOD_WIN,
            std::mem::transmute::<VIRTUAL_KEY, u16>(VK_V) as u32,
        ) {
            println!("Failed to register hotkey");
        }
        // TODO: call UnregisterHotKey?

        // Add this window as a clipboard format listener
        // TODO: add retries/backoff and fail after some time
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
                let cb_data_handle = if let Ok(h) =
                    GetClipboardData(std::mem::transmute::<CLIPBOARD_FORMAT, u16>(CF_TEXT) as u32)
                {
                    h
                } else {
                    println!("Clipboard data was not CF_TEXT or handle fetch failed"); // TODO: trace log

                    // TODO: use closure to do this so that we can unconditionally CloseClipboard with only one line
                    while let Err(_) = CloseClipboard() {
                        println!("Failed to close clipboard. Retrying...");
                    }
                    return LRESULT(1); // TODO: error codes?
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
                let data = match std::ffi::CStr::from_ptr(data as *const _).to_str() {
                    Ok(data) => data,
                    Err(e) => {
                        println!("Failed to convert data to UTF-8: {e}");
                        panic!();
                    }
                };
                // println!("COPIED: {data}"); // TODO: delete or make trace log
                // TODO: apply a size limit
                clip_store_op(|store| store.add_clip(data.to_owned()));

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
                // Coerce wparam into a hotkey ID and make sure it matches the one we registered
                let hotkey_id: isize = std::mem::transmute(wparam);
                let hotkey_id: i32 = i32::try_from(hotkey_id)
                    .expect("Failed to cast hotkey_id from isize down to i32");
                debug_assert!(hotkey_id == HOTKEY_ID_WIN_V);

                // Dump clipboard, for now
                println!("Dumping clipboard:");
                clip_store_op(|store| store.dump());
                // TODO: just do this for now to test it out
                let query = "abc";
                println!("Matching clipboard on \"{query}\":");
                clip_store_op(|store| store.get_matches(query));

                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
