mod store;
mod util;
mod win;

use store::ClipboardStore;

pub(crate) static mut CLIP_STORE: std::sync::OnceLock<ClipboardStore> = std::sync::OnceLock::new();

fn main() {
    // Create ClipboardStore and save in OnceLock
    unsafe {
        if let Err(_) = CLIP_STORE.set(ClipboardStore::new()) {
            panic!("Failed to setup ClipboardStore");
        }
    }

    // Run windows listener loop
    crate::win::run_loop();
}
