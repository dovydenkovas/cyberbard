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
//   along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::storage::source::Source;
use crate::storage::stream::Stream;

/// Audio trait. Describe Track and Composition interface.
pub trait Audio {
    fn get_source(&self) -> Result<Box<dyn Source>, AudioError>;
    fn set_source(&mut self, source: Box<dyn Source>);
    fn get_volume(&self) -> u8;
    fn set_volume(&mut self, volume: u8);
    fn is_looped(&self) -> bool;
    fn looped(&mut self, looped: bool);
    fn get_stream(&self) -> Option<Stream>;
    fn insert_audio(&mut self, index: usize, audio: Box<dyn Audio>) -> Result<(), AudioError>;
    fn erase_audio(&mut self, index: usize) -> Result<(), AudioError>;
    fn get_audio(&self, index: usize) -> Result<Box<dyn Audio>, AudioError>;
    fn audio_count(&self) -> usize;
    fn clone_box(&self) -> Box<dyn Audio>;
}

impl Clone for Box<dyn Audio> {
    fn clone(&self) -> Box<dyn Audio> {
        self.clone_box()
    }
}

#[derive(Debug, PartialEq)]
pub enum AudioError {
    NotAComposition,
    NotATrack,
    OutOfRange,
}
