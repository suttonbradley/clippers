use chrono::Utc;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use log::trace;
use sled;

const SCORE_THRESHOLD: i64 = 10;

pub(crate) struct ClipboardStore {
    clips: sled::Db,
    // TODO: store pinned
}

impl ClipboardStore {
    pub(crate) fn new() -> Self {
        // Create database
        Self {
            clips: sled::open("clipboard").expect("Could not open database"),
        }
    }

    pub(crate) fn add_clip(&mut self, data: &str) {
        // TODO: dedup
        trace!("Adding clip to DB: \"{}\"", data);
        self.clips
            .insert(Utc::now().to_string(), data)
            .expect("Failed to insert into DB");
    }

    // TODO: delete
    pub(crate) fn dump(&self) {
        for clip in self.clips.iter() {
            if let Ok((k, v)) = clip {
                trace!(
                    "{}: {}",
                    std::str::from_utf8(k.as_ref()).unwrap(),
                    std::str::from_utf8(v.as_ref()).unwrap()
                );
            }
        }
    }

    // TODO: should actually return something, for now just print
    pub(crate) fn get_matches(&self, query: &str) {
        let matcher = SkimMatcherV2::default().ignore_case(); // TODO: make case ignore configurable?

        // TODO: this creates an owned string copied out of the database, not performant
        let mut matches: Vec<(i64, String)> = self
            .clips
            .iter()
            .filter_map(|x| {
                x.ok().map_or(None, |(_, v)| {
                    // Make string from DB entry
                    let v = std::str::from_utf8(v.as_ref()).unwrap().to_owned();
                    // Calculate score and filter out scores under threshold
                    let score = matcher.fuzzy_match(v.as_str(), query).unwrap_or(0);
                    if score < SCORE_THRESHOLD {
                        None
                    } else {
                        Some((score, v))
                    }
                })
            })
            .collect();
        matches.sort_by_key(|x| std::cmp::Reverse(x.0));

        // TODO: delete
        trace!("Matches for \"{}\": {:?}", query, matches);
    }

    // TODO: impl
    // fn clear_old(&mut self) {
    //     unimplemented!();
    // }
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::path::Path;
    use std::time::Instant;

    use rand::distributions::Alphanumeric;
    use rand::Rng;

    use super::*;

    fn rand_string(range: std::ops::Range<usize>) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(rand::thread_rng().gen_range(range))
            .map(char::from)
            .collect()
    }

    #[test]
    fn test() {
        // TODO: currently, get_matches prints, which affects perf -- comment it out for the test until fixed
        // Parameters
        const NUM_QUERIES: usize = 1_000; // searches per DB size
        const MIN_DB_ENTRY_LEN: usize = 10;
        const MAX_DB_ENTRY_LEN: usize = 2000; // TODO: vary this over testing iters
        const MIN_QUERY_LEN: usize = 1;
        const MAX_QUERY_LEN: usize = 10; // TODO: vary this over testing iters

        // Create db with random ending, removing old one
        let db_name = format!("test-clipboard-{}", rand::thread_rng().gen::<u16>());
        let db_name = Path::new(db_name.as_str());
        let mut db = ClipboardStore {
            clips: sled::open(&db_name).expect("Could not open database"),
        };

        for num_elements in [10, 100 /*, 1_000, 10_000*/] {
            // Add random clips to get to desired size
            for _ in 0..(num_elements - db.clips.len()) {
                db.add_clip(rand_string(MIN_DB_ENTRY_LEN..MAX_DB_ENTRY_LEN).as_str());
            }

            // Search for random string many times to get average search time
            let start_time = Instant::now();
            let queries: Vec<String> = (0..NUM_QUERIES)
                .map(|_| rand_string(MIN_QUERY_LEN..MAX_QUERY_LEN))
                .collect();
            for query in queries {
                db.get_matches(&query);
            }
            let avg_duration = (Instant::now() - start_time) / NUM_QUERIES.try_into().unwrap();
            println!("With {num_elements} elements, took {avg_duration:?} seconds per query");
            println!(
                "With {num_elements} elements, size of DB: {}",
                db.clips.size_on_disk().unwrap()
            );
        }

        fs::remove_dir_all(&db_name).expect("Failed to remove created test db!");
    }
}
