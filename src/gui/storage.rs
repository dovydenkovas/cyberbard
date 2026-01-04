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

use std::{cell::RefCell, path::PathBuf, rc::Rc};

use egui::{Label, Sense, Ui};
use rfd::FileDialog;

use crate::{
    audio::track::Track,
    gui::events::{Event, Events},
    storage::storage::{Storage, StorageCredentials},
};

struct Music {
    shown: bool,
    index: usize,
    title: String,
    tags: Vec<String>,
}

impl Music {
    fn new(shown: bool, index: usize, title: String, tags: Vec<String>) -> Music {
        Music {
            shown,
            index,
            title: title.to_string(),
            tags: tags.iter().map(|x| x.to_string()).collect(),
        }
    }
}

pub struct StorageWidget {
    caption: String,
    search_pattern: String,
    music: Vec<Music>,
    storage: Rc<RefCell<dyn Storage>>,
}

impl StorageWidget {
    pub fn new(storage: Rc<RefCell<dyn Storage>>) -> StorageWidget {
        let mut widget = StorageWidget {
            caption: "".to_string(),
            search_pattern: "".to_string(),
            music: vec![],
            storage,
        };
        widget.sync_with_storage();
        widget
    }

    pub fn sync_with_storage(&mut self) {
        self.music.clear();
        let n = self.storage.borrow().len();
        for i in 0..n {
            self.music.push(Music::new(
                true,
                i,
                self.storage.borrow().get(i).unwrap().get_title(),
                vec![],
            ));
        }
        self.caption = self.storage.borrow().get_caption();
        self.search();
    }

    fn open_project(&mut self, events: &mut Events) {
        println!("Open project");
        let path = FileDialog::new()
            .set_title("Выбор каталога с музыкой и файлами игры")
            .pick_folder();

        if let Some(path) = path {
            events.push_back(Event::SetupStorage {
                credentials: StorageCredentials::Local { path },
            });
        }
    }

    fn save_project(&self, events: &mut Events) {
        println!("Save project");
        events.push_back(Event::SaveProject {
            path: PathBuf::from("test.toml"),
        });
    }

    fn search(&mut self) {
        let pattern = self.search_pattern.as_str();
        for source in &mut self.music {
            source.shown = source.title.contains(pattern)
                || source.tags.iter().any(|tag| tag.contains(pattern));
        }
    }

    fn send_source_to_player(&self, source: &Music, events: &mut Events) {
        let audio = Rc::new(RefCell::new(Track::new(
            self.storage.borrow().get(source.index).unwrap(),
        )));
        events.push_back(Event::Play { audio });
    }

    fn send_source_to_map(&self, source: &Music, events: &mut Events) {
        let source = self.storage.borrow().get(source.index).unwrap();
        let audio = Rc::new(RefCell::new(Track::new(source)));
        events.push_back(Event::AddAudioToComposition { audio });
    }

    pub fn update(&mut self, _ctx: &egui::Context, ui: &mut Ui, events: &mut Events) {
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            if ui.button("🗁".to_string()).clicked() {
                self.open_project(events)
            };
            if ui.button("💾".to_string()).clicked() {
                self.save_project(events)
            };
            ui.vertical_centered(|ui| {
                ui.heading(&self.caption);
            });
        });

        ui.add_space(10.0);
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("🔎".to_string());
            let search =
                egui::TextEdit::singleline(&mut self.search_pattern).hint_text("Название или тег");
            if ui.add(search).changed() {
                self.search();
            }
        });

        ui.add_space(10.0);
        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show(ui, |ui| {
                for source in &self.music {
                    if source.shown {
                        self.render_music(ui, source, events);
                    }
                }
            });
    }

    fn render_music(&self, ui: &mut Ui, source: &Music, events: &mut Events) {
        ui.horizontal(|ui| {
            let title_label = Label::new(&source.title).sense(Sense::click()).selectable(false);
            let ui_label = ui.add(title_label);
            if ui_label.clicked() {
               self.send_source_to_player(source, events);
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(20.0);
                if ui.button("+".to_string()).clicked() {
                    self.send_source_to_map(source, events);
                }
                for tag in &source.tags {
                    let frame = egui::Frame::new()
                        .fill(egui::Color32::from_rgb(0, 40, 0))
                        .corner_radius(5)
                        .inner_margin(egui::Margin::same(2));

                    frame.show(ui, |ui| {
                        ui.add(Label::new(tag));
                    });
                }
            });
        });
        ui.add_space(3.0);
    }
}
