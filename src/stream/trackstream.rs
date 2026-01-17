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

use rodio::Sink;

use super::Opener;

pub struct TrackStream {
    source: Box<dyn Opener + Send>,
    volume: f32,
}

impl TrackStream {
    pub fn new(source: Box<dyn Opener + Send>, volume: f32) -> TrackStream {
        TrackStream { source, volume }
    }

    pub fn reset_sink(&mut self, sink: &mut Sink) -> Result<(), Box<dyn std::error::Error>> {
        let source = self.source.source()?;
        sink.clear();
        sink.append(source);
        Ok(())
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, v: f32) {
        self.volume = v;
    }

    pub fn total_duration(&self) -> f32 {
        self.source.total_duration()
    }
}
