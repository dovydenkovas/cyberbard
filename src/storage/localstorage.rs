//   Cyberbard music player for board role-playing games.
//   Copyright (C) 2025  Aleksandr Dovydenkov <asd@altlinux.org>
//
//   This program is free software: you can redistribute it and/or modify
//   it under the terms of the GNU General Public License as published by
//   the Free Software Foundation, either version 3 of the License, or
//   (at your option) any later version.
//
//   This program is distributed in the hope that it will be useful,
//   but WITHOUT ANY WARRANTY; without even the implied warranty of
//   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//   GNU General Public License for more details.
//
//   You should have received a copy of the GNU General Public License
//   along with this program.  If not, see <https://www.gnu.org/licenses/>

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::storage::storage::StorageCredentials;
use crate::storage::stream::Stream;
use crate::storage::tag::Tag;

use super::storage::Storage;
use super::stream::Opener;
use id3::TagLike;
use rodio::Source;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

type BSource = Box<dyn super::source::Source>;

/// Storage of audio sources, that read audio files from local disk.
/// Open stream from .mp3, .ogg and so on files.
#[derive(Serialize)]
pub struct LocalStorage {
    storage_path: PathBuf,
    caption: String,
    sources: Vec<BSource>,
    tags: HashMap<String, Vec<Tag>>,
}

fn is_music_file(filename: &str) -> bool {
    filename.ends_with(".mp3")
        || filename.ends_with(".MP3")
        || filename.ends_with(".ogg")
        || filename.ends_with(".flac")
}

impl LocalStorage {
    pub fn new(storage_path: String) -> LocalStorage {
        let mut storage = LocalStorage {
            storage_path: PathBuf::from(&storage_path),
            caption: storage_path,
            sources: vec![],
            tags: HashMap::new(),
        };
        storage.load_sources();
        storage
    }
}

impl Storage for LocalStorage {
    fn load_sources(&mut self) {
        let mut sources = vec![];
        self.tags.clear();
        for entry in WalkDir::new(self.storage_path.clone()) {
            match entry {
                Ok(dir_entry) => {
                    let filename = dir_entry.path().to_string_lossy().to_string();
                    if is_music_file(&filename) {
                        let mut title: String = Path::new(&filename)
                            .file_stem()
                            .unwrap()
                            .to_string_lossy()
                            .chars()
                            .take(50)
                            .collect();

                        if let Ok(tag) = id3::Tag::read_from_path(dir_entry.path()) {
                            // todo: Add artist
                            // if let Some(artist) = tag.artist() {
                            //     println!("artist: {}", artist);
                            // }
                            if let Some(t) = tag.title() {
                                if !t.trim().is_empty() {
                                    title = t.trim().to_string();
                                }
                            }
                        }

                        sources.push(LocalSource::new(filename, title));
                    }
                }
                Err(_) => (),
            }
        }
        self.sources = sources;
    }

    fn get(&self, index: usize) -> Option<BSource> {
        self.sources.get(index).and_then(|v| Some(v.clone()))
    }

    fn len(&self) -> usize {
        self.sources.len()
    }

    fn attach_tag(&mut self, index: usize, title: String) {
        todo!()
    }

    fn unattach_tag(&mut self, index: usize, title: String) {
        todo!()
    }

    fn find_by_tag(&self, substr: String) -> Vec<usize> {
        todo!()
    }

    fn find_by_title(&self, substr: String) -> Vec<usize> {
        todo!()
    }

    fn get_caption(&self) -> String {
        self.caption.clone()
    }

    fn set_caption(&mut self, new_caption: String) {
        self.caption = new_caption
    }

    fn setup_storage(&mut self, cred: StorageCredentials) {
        match cred {
            StorageCredentials::Local { path } => {
                self.storage_path = path;
                self.load_sources();
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LocalSource {
    filename: String,
    title: String,
}

impl LocalSource {
    pub fn new(filename: String, title: String) -> Box<dyn crate::storage::source::Source> {
        Box::new(LocalSource { filename, title })
    }
}

impl super::source::Source for LocalSource {
    fn get_stream(&self) -> super::stream::Stream {
        Stream::from_source(LocalOpener::new(self.filename.clone()))
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn clone_box(&self) -> Box<dyn crate::storage::source::Source> {
        Box::new(self.clone())
    }
}

pub struct LocalOpener {
    filename: String,
    duration: f32,
}

impl LocalOpener {
    pub fn new(filename: String) -> Box<dyn Opener + Send> {
        Box::new(LocalOpener {
            filename,
            duration: 0.0,
        })
    }
}

impl Opener for LocalOpener {
    fn source(&mut self) -> Result<Box<dyn rodio::Source + Send>, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(&self.filename)?;
        let decoder = rodio::Decoder::try_from(file)?;
        self.duration = decoder.total_duration().unwrap_or_default().as_secs_f32();
        Ok(Box::new(decoder))
    }

    fn total_duration(&self) -> f32 {
        self.duration
    }
}

#[cfg(test)]
mod tests {
    // use crate::storage::{
    //     localstorage::{LocalSource, LocalStorage},
    //     source::Source,
    //     storage::Storage,
    // };

    // fn gen_source() -> LocalSource {
    //     LocalSource {}
    // }

    // /// Create 5 random noises in test_music directory.
    // fn gen_storage() {}

    // #[test]
    // fn empty_source() {
    //     let storage: Box<dyn Storage> = Box::new(LocalStorage::new("empty_source".to_string()));
    //     assert_eq!(0, storage.len());
    // }

    // #[test]
    // fn music_source() {
    //     todo!("Mock directory with files");
    //     gen_storage();
    //     let storage: Box<dyn Storage> = Box::new(LocalStorage::new("test_music".to_string()));
    //     assert_eq!(5, storage.len());
    //     todo!("Mock source file");

    //     assert_eq!(None, storage.get(5));
    //     assert_eq!(None, storage.get(100));
    //     assert_eq!(_, storage.get(0));
    // }

    // #[test]
    // fn find_by_title() {
    //     todo!("Mock directory with files");
    //     let storage: Box<dyn Storage> = Box::new(LocalStorage::new("test_music".to_string()));
    //     todo!("Mock source file");

    //     assert_eq!(_, storage_find_by_title());
    //     assert_eq!(_, storage_find_by_title());
    //     assert_eq!(_, storage_find_by_title());
    // }

    // #[test]
    // fn tags() {
    //     todo!("Mock directory with files");
    //     let storage: Box<dyn Storage> = Box::new(LocalStorage::new("test_music".to_string()));
    //     todo!("Mock source file");

    //     assert_eq!(_, storage_find_by_tag());
    //     storage_attach_tag();
    //     assert_eq!(_, storage_find_by_tag());
    //     storage_attach_tag();
    //     assert_eq!(_, storage_find_by_tag());
    //     storage_unattach_tag();
    //     assert_eq!(_, storage_find_by_tag());
    // }

    // #[test]
    // fn get_stream() {
    //     todo!("Mock directory with files");
    //     let storage: Box<dyn Storage> = Box::new(LocalStorage::new("test_music".to_string()));
    //     todo!("Mock source file");

    //     let source = storage.get(0);
    //     assert!(source.get_stream()); //todo
    // }

    // fn get_title() {
    //     todo!("Mock directory with files");
    //     let storage: Box<dyn Storage> = Box::new(LocalStorage::new("test_music".to_string()));
    //     todo!("Mock source file");

    //     let source = storage.get(0);
    //     assert_eq!("", source.get_title()); //todo
    // }

    // fn clone_box() {
    //     todo!("Mock directory with files");
    //     let storage: Box<dyn Storage> = Box::new(LocalStorage::new("test_music".to_string()));
    //     todo!("Mock source file");

    //     let source = storage.get(0);
    //     let cloned = source.clone();
    //     assert_eq!(source, cloned); //todo
    //     assert_ne!(
    //         &source.as_ref().unwrap() as *const dyn Source,
    //         &cloned as *const Source
    //     );
    // }
}
