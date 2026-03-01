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
use std::path::PathBuf;

use id3::TagLike;
use walkdir::WalkDir;

use crate::storage::is_music_file;
use crate::storage::tag::Tag;
use rodio::Source;
use crate::stream::Opener;

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

pub fn load_local_sources(storage_path: &PathBuf) -> (Vec<super::source::Source>, Vec<Tag>) {
    let mut sources = Vec::new();
    let mut tags = Vec::new();

    for dir_entry in WalkDir::new(storage_path).into_iter().flatten() {
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

            let mut new_source = super::source::Source::new(filename, title);
            if let Some(i) = tags.iter().position(|p: &Tag| {p.get_text() == tag}) {
                new_source.attach_tag(i);
            } else {
                let i = tags.len();
                let tag = Tag::new(tag);
                tags.push(tag);
                new_source.attach_tag(i);
            }

            sources.push(new_source);
        }
    }

   (sources, tags)
}


#[cfg(test)]
mod tests {
    use crate::storage::{Storage, StorageCredentials};

    #[test]
    fn storage_local_open() {
        let mut s = Storage::new();
        s.setup_storage(StorageCredentials::Local("test/".into()));
        assert_eq!("New storage", s.get_caption());
        assert_eq!(1, s.len());
        assert_eq!(vec![0], s.find("Metal".into()));
        assert_eq!("test", s.get_tags(0)[0].get_text());
        assert!(s.all_tags(0)[0].1);
    }

    #[test]
    fn storage_local_tags() {
        let mut s = Storage::new();
        s.setup_storage(StorageCredentials::Local("test/".into()));
        s.add_tag();
        let text = s.all_tags(0)[1].0.get_text();
        assert!(!s.all_tags(0)[1].1);
        s.rename_tag(text, "tag".into());
        s.attach_tag(0, "tag".into());
        s.remove_tag("test".into());
        assert_eq!(vec![0], s.find("Metal".into()));
        assert_eq!("tag", s.get_tags(0)[0].get_text());
        assert!(s.all_tags(0)[0].1);
    }

    #[test]
    fn storage_opener() {
        let mut s = Storage::new();
        s.setup_storage(StorageCredentials::Local("test/".into()));
        s.get(0).unwrap().get_stream().unwrap().play();
        s.get(0).unwrap().get_stream().unwrap().stop();
    }
}