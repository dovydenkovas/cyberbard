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

/// Audio stream. Low level structure to send audio to player.
use rodio::Sink;

pub struct Stream {
    sinks: Vec<Sink>,
}

impl Stream {
    pub fn new() -> Stream {
        Stream { sinks: vec![] }
    }

    pub fn from_synk(sink: Sink) -> Stream {
        Stream { sinks: vec![sink] }
    }

    pub fn is_empty(&self) -> bool {
        self.sinks.len() == 0
    }

    pub fn get_sinks(self) -> Vec<Sink> {
        self.sinks
    }

    pub fn merge(&mut self, other: Stream) {
        let sinks = other.get_sinks();
        for sink in sinks {
            self.sinks.push(sink);
        }
    }
}
