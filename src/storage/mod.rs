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

pub mod localstorage;
pub mod source;
pub mod tag;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use source::Source;
use tag::Tag;

use crate::{colors, storage::localstorage::load_local_sources};

#[derive(Deserialize, Serialize)]
pub enum StorageCredentials {
    Local(PathBuf),
}

/// Storage of audio sources, that read audio files from local disk.
/// Open stream from .mp3, .ogg and so on files.
#[derive(Deserialize, Serialize)]
pub struct Storage {
    title: String,
    credentials: Option<StorageCredentials>,
    sources: Vec<Source>,
    tags: Vec<Tag>,
}

impl Storage {
    pub fn new() -> Storage {
        let storage = Storage {
            title: "New storage".into(),
            credentials: None,
            sources: vec![],
            tags: vec![],
        };
        storage
    }

    pub fn get(&self, index: usize) -> Option<Source> {
        self.sources.get(index).cloned()
    }

    pub fn len(&self) -> usize {
        self.sources.len()
    }

    pub fn attach_tag(&mut self, source_index: usize, tag: String) {
        if source_index >= self.sources.len() {
            return;
        }

        if let Some(i) = self.tags.iter().position(|p: &Tag| p.get_text() == tag) {
            self.sources[source_index].attach_tag(i);
        } else {
            let i = self.tags.len();
            let tag = Tag::new(tag);
            self.tags.push(tag);
            self.sources[source_index].attach_tag(i);
        }
    }

    pub fn get_caption(&self) -> String {
        self.title.clone()
    }

    pub fn set_caption(&mut self, new_caption: String) {
        self.title = new_caption
    }

    pub fn setup_storage(&mut self, cred: StorageCredentials) {
        self.credentials = Some(cred);
        (self.sources, self.tags) = match &self.credentials.as_ref().unwrap() {
            StorageCredentials::Local(path_buf) => load_local_sources(&path_buf),
        }
    }

    pub fn unattach_tag(&mut self, index: usize, tag: String) {
        let tag_index = self.tags.iter().position(|t| t.get_text() == tag);

        if let Some(i) = tag_index {
            self.sources[index].unattach_tag(i);
        }
    }

    pub fn rename_tag(&mut self, old_name: String, new_name: String) {
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

    pub fn remove_tag(&mut self, tag_text: String) {
        if let Some(tag_index) = self.tags.iter().position(|t| t.get_text() == tag_text) {
            for source in &mut self.sources {
                source.remove_tag_and_shift_indexes(tag_index);
            }
        }
        self.tags.retain(|t| t.get_text() != tag_text);
    }

    pub fn add_tag(&mut self) {
        self.tags.push(Tag::random());
    }

    pub fn set_tag_color(&mut self, title: String, color: String) {
        for tag in &mut self.tags {
            if tag.get_text() == title {
                tag.set_color(color);
                break;
            }
        }
    }

    pub fn reverse_colors(&mut self) {
        for tag in &mut self.tags {
            let color = tag.get_color();
            let color = colors::reverse_color(color);
            tag.set_color(color);
        }
    }

    pub fn find(&self, substr: String) -> Vec<usize> {
        let pattern = substr.to_lowercase();
        let mut matched = Vec::with_capacity(self.sources.len());
        for i in 0..self.sources.len() {
            let title = &self.sources[i].get_title();
            if title.to_lowercase().contains(&pattern)
                || self.sources[i]
                    .tags()
                    .iter()
                    .any(|i| self.tags[*i].get_text().to_lowercase().contains(&pattern))
            {
                matched.push(i);
            }
        }
        matched
    }

    pub fn get_tags(&self, index: usize) -> Vec<&Tag> {
        if index >= self.sources.len() {
            return vec![];
        }

        self.sources[index]
            .tags()
            .iter()
            .filter_map(|&index| self.tags.get(index))
            .collect()
    }

    pub fn all_tags(&self, source_index: usize) -> Vec<(Tag, bool)> {
        let mut tags: Vec<(Tag, bool)> = self.tags.iter().map(|t| (t.clone(), false)).collect();

        if source_index < self.sources.len() {
            for i in self.sources[source_index].tags() {
                tags[i].1 = true;
            }
        }

        tags
    }
}

fn is_music_file(filename: &str) -> bool {
    let filename = filename.to_lowercase();
    [".mp3", ".flac", ".wav", ".ogg"]
        .iter()
        .any(|x| filename.ends_with(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_empty() {
        let mut storage = Storage::new();

        // Check all is empty
        assert!(storage.all_tags(0).is_empty());
        assert!(storage.get_tags(0).is_empty());
        assert!(storage.find("substr".into()).is_empty());
        assert!(storage.get(0).is_none());
        assert_eq!(0, storage.len());

        assert_eq!("New storage", storage.get_caption());
        storage.set_caption("title".into());
        assert_eq!("title", storage.get_caption());

        // Check no crash
        storage.reverse_colors();
        storage.remove_tag("tag".into());
        storage.rename_tag("tag1".into(), "tag2".into());
        storage.set_tag_color("tag".into(), "#ffaaff".into());
        storage.add_tag();
    }
}
