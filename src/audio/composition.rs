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

use crate::audio::audio::{Audio, AudioError};
use crate::storage::source::Source;
use crate::storage::stream::Stream;

/// Composition is container for other compositions and tracks.
/// Contains common settings for group of music and procedure summary Stream.
/// Composition implements Audio trait.
#[derive(Clone)]
pub struct Composition {
    volume: u8,
    is_looped_flag: bool,
    audios: Vec<Box<dyn Audio>>,
}

impl Composition {
    fn new() -> Composition {
        Composition {
            volume: 100,
            is_looped_flag: true,
            audios: vec![],
        }
    }
}

impl Audio for Composition {
    fn get_source(&self) -> Result<Box<dyn Source>, AudioError> {
        Err(AudioError::NotATrack)
    }

    fn set_source(&mut self, _: Box<dyn Source>) {
        // Not implemented for composition
    }

    fn get_volume(&self) -> u8 {
        self.volume
    }

    fn set_volume(&mut self, volume: u8) {
        self.volume = volume.clamp(0, 100);
    }

    fn is_looped(&self) -> bool {
        self.is_looped_flag
    }

    fn looped(&mut self, looped: bool) {
        self.is_looped_flag = looped;
    }

    fn get_stream(&self) -> Option<Stream> {
        let mut stream = Stream::new(vec![]);
        let mut is_none = true;
        for audio in self.audios.iter() {
            match audio.get_stream() {
                Some(s) => {
                    stream.merge(s);
                    is_none = false;
                }
                _ => (),
            }
        }
        if is_none { None } else { Some(stream) }
    }

    fn insert_audio(&mut self, index: usize, audio: Box<dyn Audio>) -> Result<(), AudioError> {
        match self.audios.len().cmp(&index) {
            std::cmp::Ordering::Less => Err(AudioError::OutOfRange),
            std::cmp::Ordering::Equal => {
                self.audios.push(audio);
                Ok(())
            }
            std::cmp::Ordering::Greater => {
                self.audios.insert(index, audio);
                Ok(())
            }
        }
    }

    fn erase_audio(&mut self, index: usize) -> Result<(), AudioError> {
        match self.audios.len().cmp(&index) {
            std::cmp::Ordering::Less => Err(AudioError::OutOfRange),
            std::cmp::Ordering::Equal => {
                self.audios.pop();
                Ok(())
            }
            std::cmp::Ordering::Greater => {
                self.audios.remove(index);
                Ok(())
            }
        }
    }

    fn get_audio(&self, index: usize) -> Result<Box<dyn Audio>, AudioError> {
        match self.audios.len().cmp(&index) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => Err(AudioError::OutOfRange),
            std::cmp::Ordering::Greater => Ok(self.audios[index].clone()),
        }
    }

    fn audio_count(&self) -> usize {
        self.audios.len()
    }

    fn clone_box(&self) -> Box<dyn Audio> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{audio::track::Track, storage::source::Source};

    #[derive(PartialEq, Debug, Clone)]
    struct TestSource {}
    impl Source for TestSource {
        fn get_stream(&self) -> Stream {
            Stream::new(vec![])
        }

        fn get_title(&self) -> String {
            "test".to_string()
        }

        fn clone_box(&self) -> Box<dyn Source> {
            Box::new(self.clone())
        }
    }

    fn get_track() -> Track {
        Track::new(Box::new(TestSource {}))
    }

    fn get_composition() -> Composition {
        Composition::new()
    }

    #[test]
    fn composition_create() {
        let _ = get_composition();
    }

    #[test]
    fn composition_source() {
        let c = get_composition();
        assert!(!c.get_source().is_ok());
    }

    #[test]
    fn composition_volume() {
        let mut c = get_composition();
        assert_eq!(100, c.get_volume());
        c.set_volume(42);
        assert_eq!(42, c.get_volume());
        c.set_volume(150);
        assert_eq!(100, c.get_volume());
    }

    #[test]
    fn composition_looped() {
        let mut c = get_composition();
        assert_eq!(true, c.is_looped());
        c.looped(false);
        assert_eq!(false, c.is_looped());
    }

    #[test]
    fn composition_get_stream() {
        let tr = get_track();
        let mut c = Composition::new();
        let _ = c.insert_audio(0, Box::new(tr));
        assert!(c.get_stream().unwrap().is_empty());
    }

    #[test]
    fn composition_insert_erase() {
        let mut tr = get_composition();
        assert_eq!(0, tr.audio_count());

        let tr2: Box<dyn Audio> = Box::new(get_composition());
        assert!(tr.insert_audio(0, tr2).is_ok());
        let tr3: Box<dyn Audio> = Box::new(get_composition());
        assert_eq!(Err(AudioError::OutOfRange), tr.insert_audio(10, tr3));

        let a0 = tr.get_audio(0);
        assert!(a0.is_ok());
        assert!(!tr.get_audio(10).is_ok());
        assert_eq!(1, tr.audio_count());

        assert!(tr.erase_audio(0).is_ok());
        assert_eq!(Err(AudioError::OutOfRange), tr.erase_audio(10));

        assert_eq!(0, tr.audio_count());
    }
}
