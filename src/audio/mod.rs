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

pub mod playlist;
pub mod track;

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::audio::playlist::Playlist;
use crate::audio::track::Track;
use crate::stream::Stream;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum Audio {
    Track(Track),
    Playlist(Playlist),
}

impl Audio {
    pub fn get_stream(&self) -> Result<Stream, Box<dyn Error>> {
        match self {
            Audio::Track(t) => Ok(t.get_stream()?),
            Audio::Playlist(p) => Ok(p.get_stream()?),
        }
    }

    pub fn get_volume(&self) -> f32 {
        match self {
            Audio::Track(t) => t.get_volume(),
            Audio::Playlist(p) => p.get_volume(),
        }
    }

    pub fn set_volume(&mut self, v: f32) {
        match self {
            Audio::Track(t) => t.set_volume(v),
            Audio::Playlist(p) => p.set_volume(v),
        }
    }

    pub fn get_title(&self) -> String {
        match self {
            Audio::Track(t) => t.get_title(),
            Audio::Playlist(p) => p.get_title(),
        }
    }

    pub fn set_title(&mut self, title: String) {
        match self {
            Audio::Track(t) => t.set_title(title),
            Audio::Playlist(p) => p.set_title(title),
        }
    }
}

pub type AudioCell = Rc<RefCell<Audio>>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AudioError {
    NotAPlaylist,
    NotATrack,
    OutOfRange,
}
