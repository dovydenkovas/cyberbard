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

use serde::{Deserialize, Serialize};

use crate::audio::{Audio, AudioError, RawAudio};
use crate::storage::source::Source;
use crate::stream::Stream;

/// Playlist is container for other playlists and tracks.
/// Contains common settings for group of music and procedure summary Stream.
/// Playlist implements Audio trait.
#[derive(Clone, Serialize, Deserialize)]
pub struct Playlist {
    volume: f32,
    title: String,
    threads: Vec<(String, Vec<Audio>)>,
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

    fn contains_thread(&self, caption: &str) -> bool {
        self.threads.iter().any(|th| th.0 == caption)
    }

    fn find_thread(&self, caption: &str) -> Option<usize> {
        self.threads.iter().position(|th| th.0 == caption)
    }
}

#[typetag::serde]
impl RawAudio for Playlist {
    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn set_title(&mut self, title: String) {
        self.title = title;
    }

    fn get_source(&self) -> Result<Source, AudioError> {
        Err(AudioError::NotATrack)
    }

    fn set_source(&mut self, _: Source) {
        // Not implemented for playlist
    }

    fn get_volume(&self) -> f32 {
        self.volume
    }

    fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    fn get_stream(&self) -> Result<Stream, Box<dyn std::error::Error>> {
        let mut stream = Stream::new(vec![], self.volume);

        for (_, pl) in self.threads.iter() {
            let mut substream = Stream::new(vec![], self.volume);
            for audio in pl {
                substream.merge(audio.borrow().get_stream()?);
            }
            stream.merge_parallel(substream);
        }
        Ok(stream)
    }

    fn push_thread(&mut self, caption: &str) -> Result<(), AudioError> {
        if !self.contains_thread(caption) {
            self.threads.push((caption.to_string(), Vec::new()));
        }
        Ok(())
    }

    fn remove_thread(&mut self, caption: &str) {
        self.threads.retain(|th| th.0 != caption);
    }

    fn rename_thread(&mut self, old_caption: &str, new_caption: &str) {
        if !self.contains_thread(new_caption) {
            for thread in self.threads.iter_mut() {
                if thread.0 == old_caption {
                    thread.0 = new_caption.to_string();
                }
            }
        }
    }

    fn threads(&self) -> Result<Vec<String>, AudioError> {
        Ok(self.threads.iter().map(|k| k.0.clone()).collect())
    }

    fn index_of_thread(&self, name: &str) -> usize {
        self.find_thread(name).unwrap_or(0)
    }

    fn is_thread_empty(&self, name: &str) -> bool {
        self.threads[self.find_thread(name).unwrap()].1.is_empty()
    }

    fn push_audio(&mut self, thread: &str, audio: Audio) -> Result<(), AudioError> {
        match self.find_thread(thread) {
            Some(i) => {
                self.threads[i].1.push(audio);
                Ok(())
            }
            None => Err(AudioError::OutOfRange),
        }
    }

    fn remove_audio(&mut self, thread: &str, index: usize) -> Result<(), AudioError> {
        match self.find_thread(thread) {
            Some(i) => {
                self.threads[i].1.remove(index);
                Ok(())
            }
            None => Err(AudioError::OutOfRange),
        }
    }

    fn get_audio(&self, thread: &str, index: usize) -> Result<Audio, AudioError> {
        if !self.contains_thread(thread) {
            return Err(AudioError::OutOfRange);
        }

        match self.threads[self.find_thread(thread).unwrap()]
            .1
            .len()
            .cmp(&index)
        {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => Err(AudioError::OutOfRange),
            std::cmp::Ordering::Greater => {
                Ok(self.threads[self.find_thread(thread).unwrap()].1[index].clone())
            }
        }
    }

    fn audio_count(&self, thread: &str) -> usize {
        match self.find_thread(thread) {
            Some(i) => self.threads[i].1.len(),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    #[test]
    fn track() {
        let mut playlist = Playlist::new();

        assert_eq!("My cool playlist", playlist.get_title());
        playlist.set_title("title 2".into());
        assert_eq!("title 2", playlist.get_title());

        assert!(playlist.get_source().is_err());

        assert_eq!(1.0, playlist.get_volume());
        playlist.set_volume(0.6);
        assert_eq!(0.6, playlist.get_volume());
        playlist.set_volume(-0.2);
        assert_eq!(0.0, playlist.get_volume());
        playlist.set_volume(1.2);
        assert_eq!(1.0, playlist.get_volume());

        assert!(playlist.get_stream().unwrap().is_empty());

        assert!(playlist.push_thread("tread".into()).is_ok());
        assert!(playlist.push_thread("thread 1".into()).is_ok());
        assert!(playlist.push_thread("thread 2".into()).is_ok());

        playlist.rename_thread("tread".into(), "thread".into());
        playlist.rename_thread("12".into(), "14".into());

        playlist.remove_thread("tread 1".into());
        playlist.remove_thread("thread 2".into());

        assert_eq!(vec!["thread".to_string(), "thread 1".to_string()], playlist.threads().unwrap());

        assert_eq!(1, playlist.index_of_thread("thread 1".into()));

        assert!(playlist.is_thread_empty("thread 1".into()));

        let audio = Playlist::new();
        assert!(playlist.push_audio("asdfasd", Rc::new(RefCell::new(Box::new(audio)))).is_err());
        let audio = Playlist::new();
        assert!(playlist.push_audio("thread 1", Rc::new(RefCell::new(Box::new(audio)))).is_ok());

        let audio = Playlist::new();
        assert!(playlist.push_audio("thread", Rc::new(RefCell::new(Box::new(audio)))).is_ok());
        assert!(!playlist.is_thread_empty("thread".into()));
        assert_eq!(1, playlist.audio_count("thread"));

        assert!(playlist.remove_audio("thread", 0).is_ok());
        playlist.get_audio("thread 1", 0).unwrap();
    }
}