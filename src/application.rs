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

use std::{cell::RefCell, rc::Rc};

use crate::{
    Map, Player, Storage,
    audio::{audio::Audio, composition::Composition},
    storage::storage::StorageCredentials,
};

pub struct Application {
    storage: Rc<RefCell<dyn Storage>>,
    map: Rc<RefCell<Map>>,
    player: Rc<RefCell<Player>>,
    selected_compostion: Rc<RefCell<Option<Rc<RefCell<dyn Audio>>>>>,
}

impl Application {
    pub fn new(
        storage: Rc<RefCell<dyn Storage>>,
        map: Rc<RefCell<Map>>,
        player: Rc<RefCell<Player>>,
    ) -> Application {
        player
            .borrow_mut()
            .set_stream(storage.borrow().get(1).unwrap().get_stream());
        Application {
            storage,
            map,
            player,
            selected_compostion: Rc::new(RefCell::new(None)),
        }
    }

    pub fn get_storage(&self) -> Rc<RefCell<dyn Storage>> {
        Rc::clone(&self.storage)
    }

    pub fn setup_storage(&mut self, credentials: StorageCredentials) {
        self.storage.borrow_mut().setup_storage(credentials);
    }

    pub fn get_player(&self) -> Rc<RefCell<Player>> {
        Rc::clone(&self.player)
    }

    pub fn get_map(&self) -> Rc<RefCell<Map>> {
        Rc::clone(&self.map)
    }

    pub fn get_selected_composition(&self) -> Rc<RefCell<Option<Rc<RefCell<dyn Audio>>>>> {
        Rc::clone(&self.selected_compostion)
    }

    pub fn set_selected_composition(&self, comp: Option<Rc<RefCell<dyn Audio>>>) {
        self.selected_compostion.replace(comp);
    }

    pub fn player_set_audio(&mut self, audio: Rc<RefCell<dyn Audio>>) {
        if let Some(s) = audio.borrow().get_stream() {
            self.player.borrow_mut().set_stream(s);
        }
    }

    pub fn player_play(&mut self) {
        self.player.borrow_mut().play();
    }

    pub fn player_pause(&mut self) {
        self.player.borrow_mut().pause();
    }

    pub fn player_stop(&mut self) {
        self.player.borrow_mut().stop();
    }

    pub fn player_set_volume(&mut self, volume: f32) {
        self.player.borrow_mut().set_volume(volume);
    }

    pub fn map_add_composition(&mut self) {
        self.map.borrow_mut().push_new_audio();
    }
}
