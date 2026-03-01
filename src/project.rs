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

use std::path::Path;

use crate::{audio::playlist::Playlist, scene::Scene, storage::Storage};


pub struct Project {
    storages: Vec<Storage>,
    root_scene: Scene,
    playlists: Vec<Playlist>
}


impl Project {
    pub fn new() -> Project {
        // create tmp directory
        todo!()
    }

    pub fn open(path: &Path) -> Project {
        // create tmp directory from archive
        // deserialize project.yaml
        todo!()
    }

    pub fn save(&self, path: &Path) {
        // serialize project.yaml
        // archive tmp directory
        todo!()
    }

    pub fn close(&mut self) {
        // close tmp directory
        todo!()
    }

    // other methods from application.rs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "todo"]
    fn project() {
    }
}