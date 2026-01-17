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

use std::path::{Path, PathBuf};

use crate::storage::storage::StorageCredentials;
use crate::storage::tag::Tag;
use crate::stream::stream::Stream;

use super::storage::Storage;
use crate::stream::Opener;
use id3::TagLike;
use rodio::Source;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

type BSource = Box<dyn super::source::Source>;

type TagIndexes = Vec<usize>;

/// Storage of audio sources, that read audio files from local disk.
/// Open stream from .mp3, .ogg and so on files.
#[derive(Deserialize, Serialize)]
pub struct LocalStorage {
    storage_path: PathBuf,
    caption: String,
    sources: Vec<BSource>,
    tags: Vec<Tag>,
    sources_tags: Vec<TagIndexes>,
}

fn is_music_file(filename: &str) -> bool {
    let filename = filename.to_lowercase();
    vec![".mp3", ".flac", ".wav", ".ogg"]
        .iter()
        .any(|x| filename.ends_with(x))
}

impl LocalStorage {
    pub fn new(storage_path: String) -> LocalStorage {
        let mut storage = LocalStorage {
            storage_path: PathBuf::from(&storage_path),
            caption: storage_path,
            sources: vec![],
            tags: vec![],
            sources_tags: vec![],
        };
        storage.load_sources();
        storage
    }
}

#[typetag::serde]
impl Storage for LocalStorage {
    fn load_sources(&mut self) {
        self.sources.clear();
        self.sources_tags.clear();
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
                            // TODO: Add artist
                            // if let Some(artist) = tag.artist() {
                            //     println!("artist: {}", artist);
                            // }
                            if let Some(t) = tag.title() {
                                if !t.trim().is_empty() {
                                    title = t.trim().to_string();
                                }
                            }
                        }
                        let tag = dir_entry
                            .path()
                            .parent()
                            .unwrap()
                            .components()
                            .last()
                            .unwrap()
                            .as_os_str()
                            .to_string_lossy()
                            .to_string();
                        self.sources.push(LocalSource::new(filename, title));
                        self.attach_tag(self.sources.len() - 1, tag);
                    }
                }
                Err(_) => (),
            }
        }
    }

    fn get(&self, index: usize) -> Option<BSource> {
        self.sources.get(index).and_then(|v| Some(v.clone()))
    }

    fn len(&self) -> usize {
        self.sources.len()
    }

    fn attach_tag(&mut self, index: usize, tag: String) {
        if self.sources.len() != self.sources_tags.len() {
            self.sources_tags.resize(self.sources.len(), Vec::new());
        }

        if index >= self.sources_tags.len() {
            return;
        }

        let i = self.tags.iter().position(|t| t.get_text() == tag);

        if let Some(i) = i {
            self.sources_tags[index].push(i);
        } else {
            self.sources_tags[index].push(self.tags.len());
            self.tags.push(Tag::new(tag));
        }
        // println!("{:?}", self.tags)
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

    fn unattach_tag(&mut self, index: usize, tag: String) {
        let i = self.tags.iter().position(|t| t.get_text() == tag);

        if let Some(i) = i {
            self.sources_tags[index].retain(|x| *x != i);
        }
    }

    fn rename_tag(&mut self, old_name: String, new_name: String) {
        // Not allowed set empty name or existing name.
        if new_name.trim().len() == 0
            || self
                .tags
                .iter()
                .find(|t| t.get_text() == new_name)
                .is_some()
        {
            return;
        }

        for tag in &mut self.tags {
            if tag.get_text() == old_name {
                tag.set_text(new_name);
                break;
            }
        }
    }

    fn remove_tag(&mut self, name: String) {
        if let Some(index) = self.tags.iter().position(|t| t.get_text() == name) {
            for tags in &mut self.sources_tags {
                tags.retain(|&i| i != index);
                for i in 0..tags.len() {
                    if tags[i] > index {
                        tags[i] -= 1;
                    }
                }
            }
        }
        self.tags.retain(|t| t.get_text() != name);
    }

    fn add_tag(&mut self) {
        self.tags.push(Tag::random());
    }

    fn set_tag_color(&mut self, title: String, color: String) {
        for tag in &mut self.tags {
            if tag.get_text() == title {
                tag.set_color(color);
                break;
            }
        }
    }

    fn find(&self, substr: String) -> Vec<usize> {
        let pattern = substr.to_lowercase();
        let mut matched = Vec::with_capacity(self.sources.len());
        for i in 0..self.sources.len() {
            let title = &self.sources[i].get_title();
            if title.to_lowercase().contains(&pattern)
                || self.sources_tags[i]
                    .iter()
                    .any(|i| self.tags[*i].get_text().to_lowercase().contains(&pattern))
            {
                matched.push(i);
            }
        }
        matched
    }

    fn get_tags(&self, index: usize) -> Vec<&Tag> {
        if index >= self.sources.len() {
            return vec![];
        }
        self.sources_tags[index]
            .iter()
            .map(|i| &self.tags[*i])
            .collect()
    }

    fn all_tags(&self, source_index: usize) -> Vec<(Tag, bool)> {
        let mut res = vec![];
        for i in 0..self.tags.len() {
            res.push((
                self.tags[i].clone(),
                self.sources_tags[source_index].contains(&i),
            ));
        }
        res
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

#[typetag::serde]
impl super::source::Source for LocalSource {
    fn get_stream(&self) -> Stream {
        Stream::from_source(LocalOpener::new(self.filename.clone()), 100.0)
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
