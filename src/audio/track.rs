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

use crate::storage::source::Source;
use crate::stream::Stream;

/// Track is container one Stream and it's settings.
/// Track implements Audio trait.
#[derive(Clone, Serialize, Deserialize, PartialEq)]
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

impl Track {
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn get_stream(&self) -> Result<Stream, Box<dyn std::error::Error>> {
        let mut s = self.source.get_stream()?;
        s.set_partial_volume(self.volume, 0, 0);
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn track() {
        let source = Source::new("filename".into(), "title".into());
        let track = Track::new(source);
        assert!(track.get_stream().is_err());
    }

    #[test]
    fn track_set_title() {
        let source = Source::new("filename".into(), "title".into());
        let mut track = Track::new(source);

        assert_eq!("title", track.get_title());
        track.set_title("title modified".into());
        assert_eq!("title modified", track.get_title());
    }

    #[test]
    fn track_set_volume_bounds() {
        let source = Source::new("filename".into(), "title".into());
        let mut track = Track::new(source);

        assert_eq!(1.0, track.get_volume());
        track.set_volume(1.5);
        assert_eq!(1.0, track.get_volume());
        track.set_volume(-1.5);
        assert_eq!(0.0, track.get_volume());

        // Test upper bound
        track.set_volume(2.0);
        assert_eq!(1.0, track.get_volume());

        // Test lower bound
        track.set_volume(-1.0);
        assert_eq!(0.0, track.get_volume());

        // Test non-integer values
        track.set_volume(0.75);
        assert_eq!(0.75, track.get_volume());
    }
}
