mod store;
mod util;
mod win;

use std::thread::{self, JoinHandle};

use log::trace;
use store::ClipboardStore;

pub(crate) static mut CLIP_STORE: std::sync::OnceLock<ClipboardStore> = std::sync::OnceLock::new();

pub fn init() -> JoinHandle<()> {
    // Create ClipboardStore and save in OnceLock
    unsafe {
        if let Err(_) = CLIP_STORE.set(ClipboardStore::new()) {
            panic!("Failed to setup ClipboardStore");
        }
    }
    trace!("Set up clipboard OnceLock");

    // Run windows listener loop
    unsafe {
        crate::win::init();
        trace!("Starting wndproc message receiver loop...");
        thread::spawn(|| crate::win::run_loop())
    }
}
