pub use crate::tracksfile::TracksFile;

use crate::track::Track;
use anyhow::{anyhow, Result};
use camino::{Utf8Path, Utf8PathBuf};
use log::{error, warn};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use std::sync::OnceLock;

#[derive(Debug)]
pub struct Playlist {
    path: Utf8PathBuf,
    name: String,
    tracks: Vec<Track>,

    /// Cached index for `tracks`, to avoid linear search.
    tracks_map: HashMap<Track, Vec<usize>>,

    /// Whether the playlist was modified since the last `write`.
    is_modified: bool,
}

impl Playlist {
    /// Returns the path to the playlists directory.
    fn dirname() -> &'static Utf8Path {
        static PLAYLISTS_DIR: OnceLock<Utf8PathBuf> = OnceLock::new();
        PLAYLISTS_DIR.get_or_init(|| crate::path_from(dirs::home_dir, "Music/Playlists"))
    }

    /// Returns an iterator over all playlist file paths.
    fn iter_paths() -> Result<impl Iterator<Item = Utf8PathBuf>> {
        crate::iter_paths(
            Self::dirname(),
            |x| x.is_file() && x.extension().is_some_and(|y| y == "m3u")
        )
    }

    /// Clears `track_map`, iterates through `tracks` and rebuilds it.
    fn rebuild_tracks_map(&mut self) {
        self.tracks_map.clear();
        for (i, track) in self.tracks.iter().enumerate() {
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
        for (i, track) in self.tracks.iter().enumerate() {
            if !self.tracks_map.contains_key(track) {
                return false;
            }
            if !self.tracks_map[track].contains(&i) {
                return false;
            }
        }
        for (track, indices) in self.tracks_map.iter() {
            if indices.iter().any(|&i| &self.tracks[i] != track) {
                return false;
            }
        }
        true
    }

    /// Returns the playlist name.
    pub fn name(&self) -> &String {
        &self.name
    }
}

impl TracksFile for Playlist {
    fn new<T: AsRef<Utf8Path>>(fpath: T) -> Result<Self> {
        let mut pl = Self {
            path: Utf8PathBuf::from(fpath.as_ref()),
            name: String::with_capacity(64),
            tracks: Vec::new(),
            tracks_map: HashMap::new(),
            is_modified: false,
        };
        match pl.path.file_stem() {
            Some(name) => pl.name.push_str(name),
            None => return Err(anyhow!("Failed to extract filename from '{:?}'", pl.path)),
        }

        let file = BufReader::new(File::open(&pl.path)?);
        for line in file.lines() {
            let line = match line {
                Ok(str) => str,
                Err(e) => return Err(anyhow!("Failed to read line from '{}': {}", pl.path, e)),
            };
            let track = Track::new(&line);
            if pl.tracks_map.contains_key(&track) {
                pl.tracks_map.get_mut(&track)
                    .unwrap()
                    .push(pl.tracks.len());
                pl.tracks.push(track);
            } else {
                let list = vec![pl.tracks.len()];
                pl.tracks_map.insert(track.clone(), list);
                pl.tracks.push(track);
            }
        }
        debug_assert!(pl.verify_integrity());
        Ok(pl)
    }

    fn iter() -> Option<impl Iterator<Item = Self>> {
        let it = match Self::iter_paths() {
            Ok(it) => it,
            Err(e) => {
                error!("Failed to list the playlists directory '{:?}': {}", Self::dirname(), e);
                return None;
            },
        };
        let it = it.filter_map(|path|
            match Self::new(&path) {
                Ok(playlist) => Some(playlist),
                Err(e) => {
                    warn!("Failed to read playlist '{:?}': {}, skipping", path, e);
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
        self.tracks.iter()
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
            self.tracks.iter()
                .map(|x| x.path.clone().into_string())
                .collect::<Vec<String>>()
                .join("\n")
        )?;
        self.is_modified = false;
        Ok(())
    }

    fn remove_at(&mut self, index: usize) {
        if index >= self.tracks.len() {
            warn!("Out-of-bounds remove_at requested (index: {}, len: {})", index, self.tracks.len());
            return;
        }

        // Remove index pointing at the given track from `tracks_map`
        let track = &self.tracks[index];
        // If either unwrap here fails, it means `tracks_map` got corrupt somehow
        let map_index = self.tracks_map[track].iter().position(|&x| x == index).unwrap();
        self.tracks_map.get_mut(track).unwrap().remove(map_index);
        if self.tracks_map[track].is_empty() {
            self.tracks_map.remove(track);
        }

        self.tracks.remove(index);

        // Shift all higher indices down by one
        for track in &self.tracks[index..] {
            for i in self.tracks_map.get_mut(track).unwrap() {
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

    fn repath(&mut self, edits: &HashMap<Track, Utf8PathBuf>) -> Result<()> {
        if edits.keys().any(|x| !self.tracks_map.contains_key(x)) {
            return Err(anyhow!("Repath edits contain track(s) that do not appear on the playlist"));
        }
        for (target_track, new_path) in edits {
            for &index in &self.tracks_map[target_track] {
                self.tracks[index].path = new_path.clone();
            }
            self.is_modified = true;
        }
        self.rebuild_tracks_map();
        Ok(())
    }
}
