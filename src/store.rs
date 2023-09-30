use chrono::Utc;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use sled;

pub(crate) struct ClipboardStore {
    clips: sled::Db,
    // TODO: store pinned
}

impl ClipboardStore {
    pub(crate) fn new() -> Self {
        // Create database
        let db = sled::open("clipboard").expect("Could not open database");
        Self { clips: db }
    }

    pub(crate) fn add_clip(&mut self, data: String) {
        // TODO: dedup
        self.clips
            .insert(Utc::now().to_string(), data.as_str())
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

    // TODO: should actually return something, for now just print
    pub(crate) fn get_matches(&self, query: &str) {
        let matcher = SkimMatcherV2::default();
        // TODO: this creates an owned string copied out of the database, not performant
        let mut values: Vec<(i64, String)> = self
            .clips
            .iter()
            .filter_map(|x| {
                x.ok().map(|(_, v)| {
                    let v = std::str::from_utf8(v.as_ref()).unwrap().to_owned();
                    (matcher.fuzzy_match(v.as_str(), query).unwrap_or(0), v)
                })
            })
            .collect();
        values.sort_by_key(|x| std::cmp::Reverse(x.0));

        println!("{values:?}");
    }

    fn clear_old(&mut self) {
        unimplemented!();
    }
}
