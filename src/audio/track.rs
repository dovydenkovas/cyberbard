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

/// Track is container one Stream and it's settings.
/// Composition implements Audio trait.
#[derive(Clone, Serialize, Deserialize)]
pub struct Track {
    title: String,
    volume: f32,
    is_looped_flag: bool,
    source: Box<dyn Source>,
}

impl Track {
    pub fn new(source: Box<dyn Source>) -> Track {
        let title = source.get_title();

        Track {
            volume: 1.0,
            is_looped_flag: true,
            source,
            title,
        }
    }
}

#[typetag::serde]
impl RawAudio for Track {
    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn set_title(&mut self, title: String) {
        self.title = title
    }

    fn get_source(&self) -> Result<Box<dyn Source>, AudioError> {
        Ok(self.source.clone())
    }

    fn set_source(&mut self, source: Box<dyn Source>) {
        self.source = source;
    }

    fn get_volume(&self) -> f32 {
        self.volume
    }

    fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    fn get_stream(&self) -> Option<Stream> {
        let mut s = self.source.get_stream();
        s.set_partial_volume(self.volume, 0, 0);
        Some(s)
    }

    fn push_thread(&mut self, _caption: &String) -> Result<(), AudioError> {
        Err(AudioError::NotAComposition)
    }

    fn rename_thread(&mut self, _old_caption: &String, _new_caption: &String) {
        // Not implemented for track
    }

    fn remove_thread(&mut self, _caption: &String) {
        // Not implemented for track
    }

    fn threads(&self) -> Result<Vec<String>, AudioError> {
        Err(AudioError::NotAComposition)
    }

    fn push_audio(&mut self, _playlist: &String, _audio: Audio) -> Result<(), AudioError> {
        Err(AudioError::NotAComposition)
    }

    fn remove_audio(&mut self, _playlist: &String, _index: usize) -> Result<(), AudioError> {
        Err(AudioError::NotAComposition)
    }

    fn get_audio(&self, _playlist: &String, _index: usize) -> Result<Audio, AudioError> {
        Err(AudioError::NotAComposition)
    }

    fn audio_count(&self, _playlist: &String) -> usize {
        return 0;
    }
}
