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

/// Track is container one Stream and it's settings.
/// Track implements Audio trait.
#[derive(Clone, Serialize, Deserialize)]
pub struct Track {
    title: String,
    volume: f32,
    source: Source,
}

impl Track {
    pub fn new(source: Source) -> Track {
        let title = source.get_title();

        Track {
            volume: 1.0,
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

    fn get_source(&self) -> Result<Source, AudioError> {
        Ok(self.source.clone())
    }

    fn set_source(&mut self, source: Source) {
        self.source = source;
    }

    fn get_volume(&self) -> f32 {
        self.volume
    }

    fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    fn get_stream(&self) -> Stream {
        let mut s = self.source.get_stream();
        s.set_partial_volume(self.volume, 0, 0);
        s
    }

    fn push_thread(&mut self, _caption: &str) -> Result<(), AudioError> {
        Err(AudioError::NotAPlaylist)
    }

    fn rename_thread(&mut self, _old_caption: &str, _new_caption: &str) {
        // Not implemented for track
    }

    fn remove_thread(&mut self, _caption: &str) {
        // Not implemented for track
    }

    fn threads(&self) -> Result<Vec<String>, AudioError> {
        Err(AudioError::NotAPlaylist)
    }

    fn index_of_thread(&self, _name: &str) -> usize {
        0
    }

    fn is_thread_empty(&self, _name: &str) -> bool {
        true
    }

    fn push_audio(&mut self, _thread: &str, _audio: Audio) -> Result<(), AudioError> {
        Err(AudioError::NotAPlaylist)
    }

    fn remove_audio(&mut self, _thread: &str, _index: usize) -> Result<(), AudioError> {
        Err(AudioError::NotAPlaylist)
    }

    fn get_audio(&self, _thread: &str, _index: usize) -> Result<Audio, AudioError> {
        Err(AudioError::NotAPlaylist)
    }

    fn audio_count(&self, _thread: &str) -> usize {
        0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn track() {
        let source = Source::new("filename".into(), "title".into());
        let mut track = Track::new(source);

        assert_eq!("title", track.get_title());
        track.set_title("title 2".into());
        assert_eq!("title 2", track.get_title());

        assert!(track.get_source().is_ok());

        assert_eq!(1.0, track.get_volume());
        track.set_volume(0.6);
        assert_eq!(0.6, track.get_volume());
        track.set_volume(-0.2);
        assert_eq!(0.0, track.get_volume());
        track.set_volume(1.2);
        assert_eq!(1.0, track.get_volume());

        assert!(track.get_stream().is_empty());
    }
}