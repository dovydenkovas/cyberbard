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

use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub dark_theme: bool,
    pub default_size: [f32; 2],
}

impl Settings {
    pub fn new() -> Settings {
        match std::fs::read_to_string("settings.toml") {
            Ok(s) => {
                match toml::from_str(&s) {
                    Ok(settings) => settings,
                    Err(_) => Settings::default(),
                }
            },
            Err(_) => Settings::default()
        }
    }

    pub fn save(&self) {
        let s = toml::to_string_pretty(&self).unwrap();
        let _ = fs::write("settings.toml", s);
    }
}

impl Default for Settings {
    fn default() -> Self {
         Self { dark_theme: true, default_size: [1200.0, 600.0]}
    }
}
