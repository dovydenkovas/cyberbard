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

use std::{cell::RefCell, rc::Rc};

use crate::{
    application::Application,
    map::Map,
    player::Player,
    storage::{localstorage::LocalStorage, Storage},
};

mod application;
mod audio;
mod colors;
mod gui;
mod map;
mod player;
mod storage;
mod stream;
mod settings;

/// Application entry point.
/// Initialize all structures and start player and application threads.
fn main() {
    let settings = Rc::new(RefCell::new(settings::Settings::new()));
    if settings.borrow().dark_theme {
        colors::set_dark();
    } else {
        colors::set_light();
    }

    // TODO: Allow startup without storage.
    let storage: Rc<RefCell<Box<dyn Storage>>> = Rc::new(RefCell::new(Box::new(
        LocalStorage::new("music".to_string()),
    )));
    let map = Rc::new(RefCell::new(Map::new(None)));
    let player = Rc::new(RefCell::new(Player::new()));
    let application = Application::new(storage, map, player);

    gui::application::run_gui(application, Rc::clone(&settings));
    settings.borrow().save();
}
