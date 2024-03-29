pub mod entry;

pub use entry::Entry;
pub use crate::tracksfile::TracksFile;

use crate::music_dir;
use crate::track::Track;
use anyhow::{anyhow, Result};
use camino::{Utf8Path, Utf8PathBuf};
use log::{error, warn};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use std::sync::OnceLock;

#[derive(Debug)]
pub struct Playcount {
    path: Utf8PathBuf,
    entries: Vec<Entry>,

    /// Cached index for `entries` which correspond to a given track.
    tracks_map: HashMap<Track, Vec<usize>>,

    /// Whether the playcount was modified since the last `write`.
    is_modified: bool,
}

impl Playcount {
    /// Returns the path to the playcount directory.
    fn playcount_dir() -> &'static Utf8Path {
        static PLAYCOUNTS_DIR: OnceLock<Utf8PathBuf> = OnceLock::new();
        PLAYCOUNTS_DIR.get_or_init(|| music_dir().join(".playcount"))
    }

    /// Returns an iterator over all playcount file paths.
    fn iter_paths() -> Result<impl Iterator<Item = Utf8PathBuf>> {
        crate::iter_paths(
            Self::playcount_dir(),
            |x| x.is_file() && x.extension().is_some_and(|y| y == "tsv")
        )
    }

    /// Clears `track_map`, iterates through `tracks` and rebuilds it.
    fn rebuild_tracks_map(&mut self) {
        self.tracks_map.clear();
        for (i, entry) in self.entries.iter().enumerate() {
            let track = &entry.track;
            if self.tracks_map.contains_key(track) {
                self.tracks_map.get_mut(track).unwrap().push(i);
            } else {
                self.tracks_map.insert(track.clone(), vec![i]);
            }
        }
        debug_assert!(self.verify_integrity());
    }

    /// Verifies the integrity of the struct. This is quite slow and intended for use with
    /// `debug_assert`.
    fn verify_integrity(&self) -> bool {
        for (i, entry) in self.entries.iter().enumerate() {
            let track = &entry.track;
            if !self.tracks_map.contains_key(track) {
                return false;
            }
            if !self.tracks_map[track].contains(&i) {
                return false;
            }
        }
        for (track, indices) in self.tracks_map.iter() {
            if indices.is_empty() {
                return false;
            }
            if indices.iter().any(|&i| &self.entries[i].track != track) {
                return false;
            }
        }
        true
    }

    /// Returns an iterator to all entries in the playcount, in order of appearance.
    /// Note that several entries may refer to the same track.
    pub fn entries(&self) -> impl Iterator<Item = &Entry> {
        self.entries.iter()
    }

    /// Merges entries corresponding to the same track by keeping only the first one and
    /// incrementing its count by the sum of the repeated ones (which are removed).
    /// Returns the number of duplicate entries that were removed.
    pub fn merge_duplicates(&mut self) -> usize {
        // Maps self.entries indices to the amounts they should be incremented by.
        let mut increments = HashMap::<usize, usize>::new();

        // A list of all indices to remove from self.entries
        let mut dupe_indices = Vec::new();

        for track in self.tracks_unique() {
            if let Some(pos) = self.track_positions(track) {
                if pos.len() > 1 {
                    dupe_indices.extend_from_slice(&pos[1..]);
                    increments.insert(
                        pos[0],
                        pos[1..].iter().map(|&x| self.entries[x].count).sum(),
                    );
                }
            }
        }

        let n_duplicates = dupe_indices.len();

        // Tally up count and remove duplicates
        if n_duplicates != 0 {
            increments.into_iter().for_each(|(index, incr)| self.entries[index].count += incr);
            dupe_indices.sort_unstable();
            dupe_indices.into_iter().rev().for_each(|x| self.remove_at(x));
            self.is_modified = true;
        }

        debug_assert!(self.verify_integrity());
        n_duplicates
    }
}

impl TracksFile for Playcount {
    fn open<T: AsRef<Utf8Path>>(fpath: T) -> Result<Self> {
        let mut pc = Self::new(fpath)?;

        let file = BufReader::new(File::open(&pc.path)?);
        for (i, line) in file.lines().enumerate() {
            let line = match line {
                Ok(str) => str,
                Err(e) => return Err(anyhow!("Failed to read line {} in '{}': {}", i, pc.path, e)),
            };
            let entry = match line.parse::<Entry>() {
                Ok(entry) => entry,
                Err(e) => {
                    warn!("Failed to parse line {} in '{}': {}, skipping", i, pc.path, e);
                    continue;
                },
            };
            if pc.tracks_map.contains_key(&entry.track) {
                pc.tracks_map.get_mut(&entry.track)
                    .unwrap()
                    .push(pc.entries.len());
                pc.entries.push(entry);
            } else {
                let list = vec![pc.entries.len()];
                pc.tracks_map.insert(entry.track.clone(), list);
                pc.entries.push(entry);
            }
        }
        debug_assert!(pc.verify_integrity());
        Ok(pc)
    }

    fn new<T: AsRef<Utf8Path>>(fpath: T) -> Result<Self> {
        Ok(Self {
            path: Utf8PathBuf::from(fpath.as_ref()),
            entries: Vec::new(),
            tracks_map: HashMap::new(),
            is_modified: false,
        })
    }

    fn open_or_new<T: AsRef<Utf8Path>>(fpath: T) -> Result<Self> where Self: Sized {
        match fpath.as_ref().exists() {
            true => Self::open(fpath),
            false => Self::new(fpath),
        }
    }

    fn iter() -> Option<impl Iterator<Item = Self>> {
        let it = match Self::iter_paths() {
            Ok(it) => it,
            Err(e) => {
                error!("Failed to list the playcounts directory '{:?}': {}", Self::playcount_dir(), e);
                return None;
            },
        };
        let it = it.filter_map(|path|
            match Self::open(&path) {
                Ok(playcount) => Some(playcount),
                Err(e) => {
                    warn!("Failed to read playcount '{:?}': {}, skipping", path, e);
                    None
                },
            }
        );
        Some(it)
    }

    fn path(&self) -> &Utf8PathBuf {
        &self.path
    }

    fn tracks(&self) -> impl Iterator<Item = &Track> {
        self.entries.iter().map(|x| &x.track)
    }

    fn tracks_unique(&self) -> impl Iterator<Item = &Track> {
        self.tracks_map.keys()
    }

    fn contains(&self, track: &Track) -> bool {
        self.tracks_map.contains_key(track)
    }

    fn track_positions(&self, track: &Track) -> Option<&Vec<usize>> {
        self.tracks_map.get(track)
    }

    fn is_modified(&self) -> bool {
        self.is_modified
    }

    fn write(&mut self) -> Result<()> {
        let mut file = File::create(&self.path)?;
        writeln!(file, "{}",
            self.entries.iter()
                .map(|x| format!("{}\t{}", x.count, x.track.path))
                .collect::<Vec<String>>()
                .join("\n")
        )?;
        self.is_modified = false;
        Ok(())
    }

    fn remove_at(&mut self, index: usize) {
        if index >= self.entries.len() {
            warn!("Out-of-bounds remove_at requested (index: {}, len: {})", index, self.entries.len());
            return;
        }

        // Remove index pointing at the given track from `tracks_map`
        let track = &self.entries[index].track;
        // If either unwrap here fails, it means `tracks_map` got corrupt somehow
        let map_index = self.tracks_map[track].iter().position(|&x| x == index).unwrap();
        self.tracks_map.get_mut(track).unwrap().remove(map_index);
        if self.tracks_map[track].is_empty() {
            self.tracks_map.remove(track);
        }

        self.entries.remove(index);

        // Shift all higher indices down by one
        for entry in &self.entries[index..] {
            for i in self.tracks_map.get_mut(&entry.track).unwrap() {
                assert!(*i != index);
                if *i > index {
                    *i -= 1;
                }
            }
        }
        self.is_modified = true;
        debug_assert!(self.verify_integrity());
    }

    fn remove_all(&mut self, track: &Track) -> usize {
        if !self.tracks_map.contains_key(track) {
            return 0;
        }
        let mut indices = self.tracks_map[track].clone();
        indices.sort_unstable();
        for index in indices.iter().rev() {
            self.remove_at(*index);
        }
        self.is_modified = true;
        indices.len()
    }

    fn bulk_rename(&mut self, edits: &HashMap<Track, Utf8PathBuf>) -> usize {
        let mut n_changed = 0usize;
        for (target_track, new_path) in edits {
            if !self.tracks_map.contains_key(target_track) {
                continue;
            }
            for &index in &self.tracks_map[target_track] {
                self.entries[index].track.path = new_path.clone();
                n_changed += 1;
            }
            self.is_modified = true;
        }
        self.rebuild_tracks_map();
        n_changed
    }
}
