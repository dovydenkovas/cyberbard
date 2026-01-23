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

use std::{cell::RefCell, fs, io, path::PathBuf, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{
    Map, Player, Storage,
    audio::{Audio, AudioCell},
    storage::StorageCredentials,
};

#[derive(Serialize, Deserialize)]
pub struct Application {
    storage: Rc<RefCell<Box<dyn Storage>>>,
    root_map: Rc<RefCell<Map>>,

    #[serde(skip)]
    player: Rc<RefCell<Player>>,
    #[serde(skip)]
    selected_compostion: AudioCell,
    #[serde(skip)]
    current_playing: AudioCell,
}

impl Application {
    pub fn new(
        storage: Rc<RefCell<Box<dyn Storage>>>,
        root_map: Rc<RefCell<Map>>,
        player: Rc<RefCell<Player>>,
    ) -> Application {
        Application {
            storage,
            root_map,
            player,
            selected_compostion: Rc::new(RefCell::new(None)),
            current_playing: Rc::new(RefCell::new(None)),
        }
    }

    pub fn get_storage(&self) -> Rc<RefCell<Box<dyn Storage>>> {
        Rc::clone(&self.storage)
    }

    pub fn setup_storage(
        &mut self,
        credentials: StorageCredentials,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match credentials {
            StorageCredentials::Local { path } => self.open_local_project(path),
        }
    }

    pub fn get_player(&self) -> Rc<RefCell<Player>> {
        Rc::clone(&self.player)
    }

    pub fn get_root_map(&self) -> Rc<RefCell<Map>> {
        Rc::clone(&self.root_map)
    }

    pub fn get_selected_composition(&self) -> AudioCell {
        Rc::clone(&self.selected_compostion)
    }

    pub fn get_current_playing(&self) -> AudioCell {
        Rc::clone(&self.current_playing)
    }

    pub fn set_selected_composition(&self, comp: Option<Audio>) {
        self.selected_compostion.replace(comp);
    }

    pub fn player_set_audio(&mut self, audio: Audio) {
        self.current_playing.replace(Some(Rc::clone(&audio)));
        if let Some(s) = audio.borrow().get_stream() {
            self.player.borrow_mut().set_stream(s);
            self.player
                .borrow_mut()
                .set_volume(audio.borrow().get_volume());
        }
    }

    pub fn player_play(&mut self) {
        self.player.borrow_mut().play();
    }

    pub fn player_set_volume(&mut self, volume: f32) {
        self.player.borrow_mut().set_volume(volume);
    }

    pub fn player_set_track_volume(&mut self, volume: f32, composition_index: usize, index: usize) {
        if Rc::ptr_eq(
            self.selected_compostion.borrow().as_ref().unwrap(),
            self.current_playing.borrow().as_ref().unwrap(),
        ) {
            self.player
                .borrow_mut()
                .set_track_volume(volume, composition_index, index);
        }
    }

    pub fn player_sync(&mut self) {
        if self.current_playing.borrow().is_some()
            && let Some(stream) = self
                .current_playing
                .borrow()
                .as_ref()
                .unwrap()
                .borrow()
                .get_stream()
        {
            self.player.borrow_mut().sync(stream);
        }
    }

    pub fn save_project(&mut self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let s = serde_yaml::to_string(self).unwrap();
        fs::write(path, s)?;
        Ok(())
    }

    pub fn open_local_project(&mut self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        match find_yaml_files(&path) {
            Ok(files) => {
                if files.is_empty() {
                    self.storage
                        .borrow_mut()
                        .setup_storage(StorageCredentials::Local { path });
                } else {
                    let s = fs::read_to_string(&files[0]).unwrap();
                    match serde_yaml::from_str::<Application>(s.as_str()) {
                        Ok(app) => {
                            self.replace(app);
                        }
                        Err(e) => return Err(Box::new(e)),
                    }
                }
            }
            Err(e) => eprintln!("Error reading directory: {}", e),
        }
        Ok(())
    }

    fn replace(&mut self, app: Application) {
        self.current_playing.replace(None);
        self.player.borrow_mut().reset();
        self.root_map = app.root_map;
        self.selected_compostion.replace(None);
        self.storage = app.storage;

        let current = Some(Rc::clone(&self.root_map));
        restore_map(&mut self.root_map, None, current);
    }
}

fn find_yaml_files(dir_path: &PathBuf) -> io::Result<Vec<std::path::PathBuf>> {
    let mut yaml_files = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file()
            && let Some(extension) = path.extension()
            && (extension == "yaml" || extension == "yml")
        {
            yaml_files.push(path);
        }
    }

    Ok(yaml_files)
}

fn restore_map(
    map: &mut Rc<RefCell<Map>>,
    parent: Option<Rc<RefCell<Map>>>,
    current: Option<Rc<RefCell<Map>>>,
) {
    map.borrow_mut().set_parent(parent);
    for m in map.borrow().iter_maps() {
        let mut m = map.borrow().get_map(m).unwrap();
        let next = Some(Rc::clone(&m));
        restore_map(&mut m, current.clone(), next);
    }
}
