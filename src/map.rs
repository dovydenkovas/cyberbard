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

use std::{
    cell::RefCell, cmp::Ordering, collections::{btree_map, BTreeMap}, path::PathBuf, rc::Rc
};

use egui::TextureHandle;
use serde::{Deserialize, Serialize};

use crate::audio::{Audio, composition::Composition};

#[derive(Serialize, Deserialize)]
pub struct Map {
    audio: Vec<Audio>,
    maps: BTreeMap<Point, Rc<RefCell<Map>>>,

    #[serde(skip)]
    parent: Option<Rc<RefCell<Map>>>,

    #[serde(skip)]
    background: Option<TextureHandle>,
    background_path: Option<PathBuf>,
}

impl Map {
    pub fn new(parent: Option<Rc<RefCell<Map>>>) -> Map {
        Map {
            audio: vec![],
            maps: BTreeMap::new(),
            parent,
            background: None,
            background_path: None,
        }
    }

    pub fn set_parent(&mut self, new: Option<Rc<RefCell<Map>>>) {
        self.parent = new;
    }

    pub fn insert_map(&mut self, point: Point, map: Rc<RefCell<Map>>) {
        self.maps.insert(point, map);
    }

    pub fn erase_map(&mut self, point: Point) {
        if self.maps.contains_key(&point) {
            self.maps.remove(&point);
        }
    }

    pub fn get_map(&self, child: &Point) -> Option<Rc<RefCell<Map>>> {
        match self.maps.get(child) {
            Some(v) => Some(Rc::clone(v)),
            None => None,
        }
    }

    pub fn iter_maps<'a>(&'a self) -> btree_map::Keys<'a, Point, Rc<RefCell<Map>>> {
        self.maps.keys()
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<Map>>> {
        match self.parent.as_ref() {
            Some(v) => Some(Rc::clone(v)),
            None => None,
        }
    }

    pub fn set_background(&mut self, path: PathBuf, handle: TextureHandle) {
        self.background_path = Some(path);
        self.background = Some(handle);
    }

    pub fn remove_background(&mut self) {
        self.background = None;
        self.background_path = None;
    }

    pub fn get_background(&self) -> Option<TextureHandle> {
        self.background.clone()
    }

    pub fn get_background_path(&self) -> Option<PathBuf> {
        self.background_path.clone()
    }

    pub fn push_new_audio(&mut self) {
        let audio: Audio = Rc::new(RefCell::new(Box::new(Composition::new())));
        self.audio.push(audio);
    }

    pub fn erase_audio(&mut self, index: usize) {
        self.audio.remove(index);
    }

    pub fn get_audio(&self, index: usize) -> Audio {
        Rc::clone(&self.audio[index])
    }

    pub fn audio_count(&self) -> usize {
        self.audio.len()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Eq for Point {}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
       Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.x * self.x + self.y * self.y).total_cmp(&(other.x * other.x + other.y * other.y))
    }

    fn max(self, other: Self) -> Self {
        if other < self { self } else { other }
    }

    fn min(self, other: Self) -> Self {
        if other < self { other } else { self }
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}
