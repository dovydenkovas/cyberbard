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

use crate::storage::StorageCredentials;
use crate::storage::tag::Tag;
use crate::stream::Stream;

use super::Storage;
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
    [".mp3", ".flac", ".wav", ".ogg"]
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
        for dir_entry in WalkDir::new(self.storage_path.clone())
            .into_iter()
            .flatten()
        {
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
                    if let Some(t) = tag.title()
                        && !t.trim().is_empty()
                    {
                        title = t.trim().to_string();
                    }
                }
                let tag = dir_entry
                    .path()
                    .parent()
                    .unwrap()
                    .components()
                    .next_back()
                    .unwrap()
                    .as_os_str()
                    .to_string_lossy()
                    .to_string();
                self.sources
                    .push(Box::new(LocalSource::new(filename, title)));
                self.attach_tag(self.sources.len() - 1, tag);
            }
        }
    }

    fn get(&self, index: usize) -> Option<BSource> {
        self.sources.get(index).cloned()
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
        if new_name.trim().is_empty() || self.tags.iter().any(|t| t.get_text() == new_name) {
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
                for tag in tags {
                    if *tag > index {
                        *tag -= 1;
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
    pub fn new(filename: String, title: String) -> LocalSource {
        LocalSource { filename, title }
    }
}

#[typetag::serde]
impl super::source::Source for LocalSource {
    fn get_stream(&self) -> Stream {
        Stream::from_source(Box::new(LocalOpener::new(self.filename.clone())), 100.0)
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
    pub fn new(filename: String) -> LocalOpener {
        LocalOpener {
            filename,
            duration: 0.0,
        }
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
