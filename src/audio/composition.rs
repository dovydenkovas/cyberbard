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

use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::audio::audio::{Audio, AudioError, RawAudio};
use crate::storage::source::Source;
use crate::storage::stream::Stream;

/// Composition is container for other compositions and tracks.
/// Contains common settings for group of music and procedure summary Stream.
/// Composition implements Audio trait.
#[derive(Clone, Serialize)]
pub struct Composition {
    volume: f32,
    is_looped_flag: bool,
    title: String,
    playlists: BTreeMap<String, Vec<Audio>>,
}

impl Composition {
    pub fn new() -> Composition {
        let title = "Мой крутой плейлист".to_string();

        Composition {
            volume: 1.0,
            is_looped_flag: true,
            playlists: BTreeMap::new(),
            title,
        }
    }
}

impl RawAudio for Composition {
    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn set_title(&mut self, title: String) {
        self.title = title;
    }

    fn get_source(&self) -> Result<Box<dyn Source>, AudioError> {
        Err(AudioError::NotATrack)
    }

    fn set_source(&mut self, _: Box<dyn Source>) {
        // Not implemented for composition
    }

    fn get_volume(&self) -> f32 {
        self.volume
    }

    fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    fn get_stream(&self) -> Option<Stream> {
        let mut stream = Stream::new(vec![], self.volume);
        let mut is_none = true;

        for pl in self.playlists.values() {
            let mut substream = Stream::new(vec![], self.volume);
            for audio in pl {
                match audio.borrow().get_stream() {
                    Some(s) => {
                        substream.merge(s);
                        is_none = false;
                    }
                    _ => (),
                }
            }
            stream.merge_parallel(substream);
        }
        if is_none { None } else { Some(stream) }
    }

    fn push_playlist(&mut self, caption: &String) -> Result<(), AudioError> {
        if !self.playlists.contains_key(caption) {
            self.playlists.insert(caption.clone(), Vec::new());
        }
        Ok(())
    }

    fn remove_playlist(&mut self, caption: &String) {
        self.playlists.remove(caption);
    }

    fn rename_playlist(&mut self, old_caption: &String, new_caption: &String) {
        if let Some(v) = self.playlists.remove(old_caption) {
            self.playlists.insert(new_caption.clone(), v);
        }
    }

    fn playlists(&self) -> Result<Vec<String>, AudioError> {
        Ok(self.playlists.keys().map(|k| k.to_string()).collect())
    }

    fn push_audio(&mut self, caption: &String, audio: Audio) -> Result<(), AudioError> {
        match self.playlists.get_mut(caption) {
            Some(v) => {
                v.push(audio);
                Ok(())
            }
            None => Err(AudioError::OutOfRange),
        }
    }

    fn remove_audio(&mut self, caption: &String, index: usize) -> Result<(), AudioError> {
        if !self.playlists.contains_key(caption) {
            return Err(AudioError::OutOfRange);
        }

        match self.playlists[caption].len().cmp(&index) {
            std::cmp::Ordering::Less => Err(AudioError::OutOfRange),
            std::cmp::Ordering::Equal => {
                self.playlists.get_mut(caption).unwrap().pop();
                Ok(())
            }
            std::cmp::Ordering::Greater => {
                self.playlists.get_mut(caption).unwrap().remove(index);
                Ok(())
            }
        }
    }

    fn get_audio(&self, caption: &String, index: usize) -> Result<Audio, AudioError> {
        if !self.playlists.contains_key(caption) {
            return Err(AudioError::OutOfRange);
        }

        match self.playlists[caption].len().cmp(&index) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => Err(AudioError::OutOfRange),
            std::cmp::Ordering::Greater => Ok(self.playlists[caption][index].clone()),
        }
    }

    fn audio_count(&self, caption: &String) -> usize {
        self.playlists.get(caption).unwrap_or(&Vec::new()).len()
    }
}
