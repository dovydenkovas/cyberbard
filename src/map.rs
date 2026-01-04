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

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::audio::{audio::Audio, composition::Composition};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize)]
pub struct Map {
    audio: Vec<Rc<RefCell<dyn Audio>>>,
    maps: HashMap<Point, Map>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            audio: vec![],
            maps: HashMap::new(),
        }
    }

    // pub fn insert_map(&mut self, point: Point, map: Map) {}
    // pub fn erase_map(&mut self, point: Point) {}
    // pub fn get_map(&mut self, phild: Point) -> Map {}
    // pub fn get_parent(&mut self) -> Map {}
    // pub fn set_background(&mut self, image: Image) {}
    // pub fn get_background(&self, image: Image) {}
    // pub fn set_logo(&mut self, image: Image) {}
    // pub fn get_logo(&self, image: Image) {}

    pub fn insert_audio(&mut self, index: usize, audio: Rc<RefCell<dyn Audio>>) {
        self.audio.insert(index, audio);
    }

    pub fn push_new_audio(&mut self) {
        let audio = Rc::new(RefCell::new(Composition::new()));
        self.audio.push(audio);
    }

    pub fn erase_audio(&mut self, index: usize) {
        self.audio.remove(index);
    }

    pub fn get_audio(&self, index: usize) -> Rc<RefCell<dyn Audio>> {
        Rc::clone(&self.audio[index])
    }

    pub fn audio_count(&self) -> usize {
        self.audio.len()
    }
}
