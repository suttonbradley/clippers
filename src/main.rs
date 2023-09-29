mod store;
mod win;

use std::path::PathBuf;

use memmap2::MmapMut;

pub(crate) static mut MMAP: std::sync::OnceLock<MmapMut> = std::sync::OnceLock::new();

fn main() {
    // Open file to store clipboard contents, save in OnceLock
    let path = PathBuf::from("clipboard");
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .expect("Failed to get");
    // TODO: want file to auto-increase in size..?
    file.set_len(10000).expect("Failed to set file size");
    unsafe {
        MMAP.set(
            MmapMut::map_mut(&file)
                .expect(format!("Failed to create MmapMut from file {:?}", file).as_str()),
        )
        .expect("Failed to set MMAP OnceLock");
    }

    // Run windows listener loop
    crate::win::run_loop();
}
