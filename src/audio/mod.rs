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

pub mod composition;
pub mod track;

use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::storage::source::Source;
use crate::stream::Stream;

pub type Audio = Rc<RefCell<Box<dyn RawAudio>>>;
pub type AudioCell = Rc<RefCell<Option<Audio>>>;

/// Audio trait. Describe Track and Composition interface.
#[typetag::serde(tag = "type")]
pub trait RawAudio: erased_serde::Serialize {
    fn get_title(&self) -> String;
    fn set_title(&mut self, title: String);
    fn get_source(&self) -> Result<Box<dyn Source>, AudioError>;
    fn set_source(&mut self, source: Box<dyn Source>);
    fn get_volume(&self) -> f32;
    fn set_volume(&mut self, volume: f32);
    fn get_stream(&self) -> Stream;

    fn push_thread(&mut self, caption: &str) -> Result<(), AudioError>;
    fn rename_thread(&mut self, old_caption: &str, new_caption: &str);
    fn remove_thread(&mut self, caption: &str);
    fn threads(&self) -> Result<Vec<String>, AudioError>;
    fn index_of_thread(&self, name: &str) -> usize;
    fn is_thread_empty(&self, name: &str) -> bool;

    fn push_audio(&mut self, thread: &str, audio: Audio) -> Result<(), AudioError>;
    fn remove_audio(&mut self, thread: &str, index: usize) -> Result<(), AudioError>;
    fn get_audio(&self, thread: &str, index: usize) -> Result<Audio, AudioError>;
    fn audio_count(&self, thread: &str) -> usize;
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AudioError {
    NotAComposition,
    NotATrack,
    OutOfRange,
}
