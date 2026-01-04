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

use std::cell::RefCell;
use std::rc::Rc;

use erased_serde::serialize_trait_object;
use serde::{Deserialize, Serialize};

use crate::storage::source::Source;
use crate::storage::stream::Stream;

/// Audio trait. Describe Track and Composition interface.
pub trait Audio: erased_serde::Serialize {
    fn get_title(&self) -> String;
    fn set_title(&mut self, title: String);
    fn get_source(&self) -> Result<Box<dyn Source>, AudioError>;
    fn set_source(&mut self, source: Box<dyn Source>);
    fn get_volume(&self) -> f32;
    fn set_volume(&mut self, volume: f32);
    fn get_stream(&self) -> Option<Stream>;
    fn insert_audio(
        &mut self,
        index: usize,
        audio: Rc<RefCell<dyn Audio>>,
    ) -> Result<(), AudioError>;
    fn push_audio(&mut self, audio: Rc<RefCell<dyn Audio>>) -> Result<(), AudioError>;
    fn erase_audio(&mut self, index: usize) -> Result<(), AudioError>;
    fn get_audio(&self, index: usize) -> Result<Rc<RefCell<dyn Audio>>, AudioError>;
    fn audio_count(&self) -> usize;
}

serialize_trait_object!(Audio);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AudioError {
    NotAComposition,
    NotATrack,
    OutOfRange,
}
