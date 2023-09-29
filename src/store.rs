use memmap2::MmapMut;

#[derive(Debug)]
pub(crate) struct ClipboardStore {
    mmap: MmapMut,
    clips: Vec<ClipboardEntry>,
}

impl ClipboardStore {
    pub(crate) fn new() -> Self {
        // Create mmap
        let path = std::path::PathBuf::from("clipboard");
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .expect("Failed to get");
        // TODO: want file to auto-increase in size..?
        file.set_len(10000).expect("Failed to set file size");

        Self {
            mmap: unsafe { MmapMut::map_mut(&file)
                .expect(format!("Failed to create MmapMut from file {:?}", file).as_str()) },
            clips: vec![],
        }
    }

    pub(crate) fn add_clip(&mut self, data: String) {
        self.clips.push(ClipboardEntry {
            created: std::time::Instant::now(),
            data,
        });
    }

    // TODO: delete
    pub(crate) fn dump(&self) {
        for clip in &self.clips {
            println!("{clip:?}");
        }
    }
}

#[derive(Debug)]
struct ClipboardEntry {
    created: std::time::Instant,
    data: String,
}
