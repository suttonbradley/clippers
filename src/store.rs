use chrono::{DateTime, Utc};
use serde::Serialize;
use {serde_json, sled};

pub(crate) struct ClipboardStore {
    clips: sled::Db,
}

impl ClipboardStore {
    pub(crate) fn new() -> Self {
        // Create database
        let db = sled::open("clipboard").expect("Could not open database");
        Self { clips: db }
    }

    pub(crate) fn add_clip(&mut self, data: String) {
        let entry = ClipboardEntry::from_data(data);
        // TODO: remove unwrap
        let entry_json = serde_json::to_string(&entry).expect("Failed to serialize to JSON");

        self.clips
            .insert(entry.created.to_string(), entry_json.as_bytes())
            .expect("Failed to insert into DB");
    }

    // TODO: delete
    pub(crate) fn dump(&self) {
        for clip in self.clips.iter() {
            if let Ok((k, v)) = clip {
                println!(
                    "{}: {}",
                    std::str::from_utf8(k.as_ref()).unwrap(),
                    std::str::from_utf8(v.as_ref()).unwrap()
                );
            }
        }
    }
}

#[derive(Serialize)]
struct ClipboardEntry {
    // TODO: might not be necessary to store this, in which case we could get rid of serde too
    created: DateTime<Utc>,
    data: String,
}

impl ClipboardEntry {
    pub(crate) fn from_data(data: String) -> Self {
        Self {
            created: Utc::now(),
            data,
        }
    }
}
