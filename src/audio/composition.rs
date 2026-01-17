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

use crate::audio::audio::{Audio, AudioError, RawAudio};
use crate::storage::source::Source;
use crate::stream::stream::Stream;

/// Composition is container for other compositions and tracks.
/// Contains common settings for group of music and procedure summary Stream.
/// Composition implements Audio trait.
#[derive(Clone, Serialize, Deserialize)]
pub struct Composition {
    volume: f32,
    is_looped_flag: bool,
    title: String,
    threads: Vec<(String, Vec<Audio>)>,
}

impl Composition {
    pub fn new() -> Composition {
        let title = "Мой крутой плейлист".to_string();

        Composition {
            volume: 1.0,
            is_looped_flag: true,
            threads: Vec::new(),
            title,
        }
    }

    fn contains_thread(&self, caption: &String) -> bool {
        self.threads
            .iter()
            .find(|th| &th.0 == caption)
            .take()
            .is_some()
    }

    fn find_thread(&self, caption: &String) -> Option<usize> {
        self.threads.iter().position(|th| &th.0 == caption)
    }
}

#[typetag::serde]
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

        for (_, pl) in self.threads.iter() {
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

    fn push_thread(&mut self, caption: &String) -> Result<(), AudioError> {
        if !self.contains_thread(caption) {
            self.threads.push((caption.clone(), Vec::new()));
        }
        Ok(())
    }

    fn remove_thread(&mut self, caption: &String) {
        self.threads.retain(|th| &th.0 != caption);
    }

    fn rename_thread(&mut self, old_caption: &String, new_caption: &String) {
        if !self.contains_thread(new_caption) {
            for thread in self.threads.iter_mut() {
                if &thread.0 == old_caption {
                    thread.0 = new_caption.clone();
                }
            }
        }
    }

    fn threads(&self) -> Result<Vec<String>, AudioError> {
        Ok(self.threads.iter().map(|k| k.0.clone()).collect())
    }

    fn push_audio(&mut self, caption: &String, audio: Audio) -> Result<(), AudioError> {
        match self.find_thread(caption) {
            Some(i) => {
                self.threads[i].1.push(audio);
                Ok(())
            }
            None => Err(AudioError::OutOfRange),
        }
    }

    fn remove_audio(&mut self, caption: &String, index: usize) -> Result<(), AudioError> {
        match self.find_thread(caption) {
            Some(i) => {
                self.threads[i].1.remove(index);
                Ok(())
            }
            None => Err(AudioError::OutOfRange),
        }
    }

    fn get_audio(&self, caption: &String, index: usize) -> Result<Audio, AudioError> {
        if !self.contains_thread(caption) {
            return Err(AudioError::OutOfRange);
        }

        match self.threads[self.find_thread(caption).unwrap()]
            .1
            .len()
            .cmp(&index)
        {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => Err(AudioError::OutOfRange),
            std::cmp::Ordering::Greater => {
                Ok(self.threads[self.find_thread(caption).unwrap()].1[index].clone())
            }
        }
    }

    fn audio_count(&self, caption: &String) -> usize {
        match self.find_thread(caption) {
            Some(i) => self.threads[i].1.len(),
            None => 0,
        }
    }
}
