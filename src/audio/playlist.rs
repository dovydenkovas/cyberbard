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

use std::cell::RefCell;
use std::rc::Rc;

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::audio::{Audio, AudioCell};
use crate::stream::Stream;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
struct PlaylistThread {
    title: String,
    tracks: Vec<AudioCell>,
}

/// Playlist is container for other playlists and tracks.
/// Contains common settings for group of music and procedure summary Stream.
/// Playlist implements Audio trait.
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Playlist {
    volume: f32,
    title: String,
    threads: Vec<PlaylistThread>,
}

impl Playlist {
    pub fn new() -> Playlist {
        let title = t!("new_playlist_name").to_string();

        Playlist {
            volume: 1.0,
            threads: Vec::new(),
            title,
        }
    }

    fn contains_thread(&self, title: &str) -> bool {
        self.threads.iter().any(|th| th.title == title)
    }

    fn find_thread(&self, title: &str) -> Option<usize> {
        self.threads.iter().position(|th| th.title == title)
    }
}

impl Playlist {
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn get_stream(&self) -> Result<Stream> {
        let mut stream = Stream::new(vec![], self.volume);

        for th in &self.threads {
            let mut substream = Stream::new(vec![], self.volume);
            for audio in &th.tracks {
                substream.merge(
                    audio
                        .borrow()
                        .get_stream()
                        .map_err(|e| anyhow!(e.to_string()))?,
                );
            }
            stream.merge_parallel(substream);
        }
        Ok(stream)
    }

    pub fn push_thread(&mut self, title: &str) {
        if !self.contains_thread(title) {
            self.threads.push(PlaylistThread {
                title: title.to_string(),
                tracks: Vec::new(),
            });
        }
    }

    pub fn remove_thread(&mut self, title: &str) {
        self.threads.retain(|th| th.title != title);
    }

    pub fn rename_thread(&mut self, old_title: &str, new_title: &str) {
        if !self.contains_thread(new_title) {
            self.threads
                .iter_mut()
                .filter(|t| t.title == old_title)
                .for_each(|t| t.title = new_title.into());
        }
    }

    pub fn threads(&self) -> impl Iterator<Item = &str> + '_ {
        self.threads.iter().map(|k| k.title.as_str())
    }

    pub fn unempty_threads(&self) -> impl Iterator<Item = &str> + '_ {
        self.threads
            .iter()
            .filter(|th| !th.tracks.is_empty())
            .map(|k| k.title.as_str())
    }

    pub fn index_of_thread(&self, name: &str) -> usize {
        self.find_thread(name).unwrap_or(0)
    }

    pub fn is_thread_empty(&self, name: &str) -> bool {
        self.threads[self.find_thread(name).unwrap()]
            .tracks
            .is_empty()
    }

    pub fn push_audio(&mut self, thread: &str, audio: Audio) -> Result<()> {
        let idx = self
            .find_thread(thread)
            .ok_or_else(|| anyhow!("thread not found: {}", thread))?;
        self.threads[idx].tracks.push(Rc::new(RefCell::new(audio)));
        Ok(())
    }

    pub fn remove_audio(&mut self, thread: &str, index: usize) -> Result<()> {
        let idx = self
            .find_thread(thread)
            .ok_or_else(|| anyhow!("thread not found: {}", thread))?;

        match self.threads[idx].tracks.len().cmp(&index) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => Err(anyhow!(
                "try to get element with index {} from thread with len {}",
                index,
                self.threads[idx].tracks.len()
            )),
            std::cmp::Ordering::Greater => {
                self.threads[idx].tracks.remove(index);
                Ok(())
            }
        }
    }

    pub fn get_audio(&self, thread: &str, index: usize) -> Result<AudioCell> {
        let idx = self
            .find_thread(thread)
            .ok_or_else(|| anyhow!("thread not found: {}", thread))?;

        match self.threads[idx].tracks.len().cmp(&index) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => Err(anyhow!(
                "try to get element with index {} from thread with len {}",
                index,
                self.threads[idx].tracks.len()
            )),
            std::cmp::Ordering::Greater => Ok(self.threads[idx].tracks[index].clone()),
        }
    }

    pub fn audio_count(&self, thread: &str) -> usize {
        match self.find_thread(thread) {
            Some(i) => self.threads[i].tracks.len(),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn track() {
        let mut playlist = Playlist::new();

        assert_eq!("My cool playlist", playlist.get_title());
        playlist.set_title("title 2".into());
        assert_eq!("title 2", playlist.get_title());

        assert_eq!(1.0, playlist.get_volume());
        playlist.set_volume(0.6);
        assert_eq!(0.6, playlist.get_volume());
        playlist.set_volume(-0.2);
        assert_eq!(0.0, playlist.get_volume());
        playlist.set_volume(1.2);
        assert_eq!(1.0, playlist.get_volume());

        playlist.push_thread("tread".into());
        playlist.push_thread("thread 1".into());
        playlist.push_thread("thread 2".into());

        playlist.rename_thread("tread".into(), "thread".into());
        playlist.rename_thread("12".into(), "14".into());

        playlist.remove_thread("tread 1".into());
        playlist.remove_thread("thread 2".into());

        assert_eq!(
            vec!["thread".to_string(), "thread 1".to_string()],
            playlist
                .threads()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );

        assert_eq!(1, playlist.index_of_thread("thread 1".into()));

        assert!(playlist.is_thread_empty("thread 1".into()));

        let audio = Audio::Playlist(Playlist::new());
        assert!(playlist.push_audio("asdfasd", audio).is_err());
        let audio = Audio::Playlist(Playlist::new());
        assert!(playlist.push_audio("thread 1", audio).is_ok());

        let audio = Audio::Playlist(Playlist::new());
        assert!(playlist.push_audio("thread", audio).is_ok());
        assert!(!playlist.is_thread_empty("thread".into()));
        assert_eq!(1, playlist.audio_count("thread"));

        assert!(playlist.remove_audio("thread", 0).is_ok());
        playlist.get_audio("thread 1", 0).unwrap();
    }
}
