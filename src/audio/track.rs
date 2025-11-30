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

/// Track is container one Stream and it's settings.
/// Composition implements Audio trait.
#[derive(Clone)]
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

impl Audio for Track {
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

    fn is_looped(&self) -> bool {
        self.is_looped_flag
    }

    fn looped(&mut self, looped: bool) {
        self.is_looped_flag = looped;
    }

    fn get_stream(&self) -> Option<Stream> {
        Some(self.source.get_stream())
    }

    fn insert_audio(&mut self, _index: usize, _audio: Box<dyn Audio>) -> Result<(), AudioError> {
        Err(AudioError::NotAComposition)
    }

    fn erase_audio(&mut self, _index: usize) -> Result<(), AudioError> {
        Err(AudioError::NotAComposition)
    }

    fn get_audio(&self, _index: usize) -> Result<Box<dyn Audio>, AudioError> {
        Err(AudioError::NotAComposition)
    }

    fn audio_count(&self) -> usize {
        return 0;
    }

    fn clone_box(&self) -> Box<dyn Audio> {
        Box::new(self.clone())
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn set_title(&mut self, title: String) {
        self.title = title
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::source::Source;

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

    #[test]
    fn track_create() {
        let _ = get_track();
    }

    #[test]
    fn track_source() {
        let mut tr = get_track();
        let s = Box::new(TestSource {});
        assert_eq!(s.get_title(), tr.get_source().unwrap().get_title());
        tr.set_source(s);
        let s = Box::new(TestSource {});
        assert_eq!(s.get_title(), tr.get_source().unwrap().get_title());
    }

    #[test]
    fn track_volume() {
        let mut tr = get_track();
        assert_eq!(1.0, tr.get_volume());
        tr.set_volume(0.42);
        assert_eq!(0.42, tr.get_volume());
        tr.set_volume(150.0);
        assert_eq!(1.0, tr.get_volume());
    }

    #[test]
    fn track_looped() {
        let mut tr = get_track();
        assert_eq!(true, tr.is_looped());
        tr.looped(false);
        assert_eq!(false, tr.is_looped());
    }

    #[test]
    fn track_get_stream() {
        let s = TestSource {};
        let tr = Track::new(Box::new(s));
        assert!(tr.get_stream().unwrap().is_empty());
    }

    #[test]
    fn track_not_a_composition() {
        let mut tr = get_track();
        let tr2: Box<dyn Audio> = Box::new(get_track());
        assert_eq!(Err(AudioError::NotAComposition), tr.insert_audio(0, tr2));
        let tr2: Box<dyn Audio> = Box::new(get_track());
        assert_eq!(Err(AudioError::NotAComposition), tr.insert_audio(10, tr2));

        assert_eq!(Err(AudioError::NotAComposition), tr.erase_audio(0));
        assert_eq!(Err(AudioError::NotAComposition), tr.erase_audio(10));

        let err = tr.get_audio(0);
        assert!(!err.is_ok());
        assert!(!tr.get_audio(10).is_ok());
        assert_eq!(0, tr.audio_count());
    }
}
