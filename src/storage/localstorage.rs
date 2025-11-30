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

use std::path::Path;

use crate::storage::source::Source;
use crate::storage::stream::Stream;

use super::storage::Storage;
use super::stream::Opener;
use walkdir::WalkDir;

type BSource = Box<dyn super::source::Source>;

/// Storage of audio sources, that read audio files from local disk.
/// Open stream from .mp3, .ogg and so on files.
pub struct LocalStorage {
    storage_path: String,
    caption: String,
    sources: Vec<BSource>,
}

fn is_music_file(filename: &str) -> bool {
    return filename.ends_with(".mp3") || filename.ends_with(".MP3") || filename.ends_with(".ogg");
}

impl LocalStorage {
    pub fn new(storage_path: String) -> LocalStorage {
        let mut storage = LocalStorage {
            storage_path: storage_path.clone(),
            caption: storage_path,
            sources: vec![],
        };
        storage.load_sources();
        storage
    }
}

impl Storage for LocalStorage {
    fn load_sources(&mut self) {
        let mut sources = vec![];
        for entry in WalkDir::new(self.storage_path.clone()) {
            match entry {
                Ok(dir_entry) => {
                    let filename = dir_entry.path().to_string_lossy().to_string();
                    if is_music_file(&filename) {
                        sources.push(LocalSource::new(filename));
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
}

#[derive(Clone)]
pub struct LocalSource {
    filename: String,
    title: String,
}

impl LocalSource {
    pub fn new(filename: String) -> Box<dyn Source> {
        let title = Path::new(&filename)
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .chars()
            .take(50)
            .collect();

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

    fn clone_box(&self) -> Box<dyn Source> {
        Box::new(self.clone())
    }
}

pub struct LocalOpener {
    filename: String,
}

impl LocalOpener {
    pub fn new(filename: String) -> Box<dyn Opener + Send> {
        Box::new(LocalOpener { filename })
    }
}

impl Opener for LocalOpener {
    fn source(&self) -> Box<dyn rodio::Source + Send> {
        let file = std::fs::File::open(&self.filename).unwrap();
        Box::new(rodio::Decoder::try_from(file).unwrap())
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
