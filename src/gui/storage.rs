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

use egui::{Label, Ui};

struct Music {
    title: String,
    tags: Vec<String>,
}

impl Music {
    fn new(title: &str, tags: Vec<&str>) -> Music {
        Music {
            title: title.to_string(),
            tags: tags.iter().map(|x| x.to_string()).collect(),
        }
    }
}

pub struct StoreWidget {
    title: String,
    search_pattern: String,
    music: Vec<Music>,
}

impl StoreWidget {
    pub fn new() -> StoreWidget {
        StoreWidget {
            title: "Властелин Колец".to_string(),
            search_pattern: "".to_string(),
            music: vec![
                Music::new("The Shire", vec!["music"]),
                Music::new("Concerning Hobbits", vec!["music"]),
                Music::new("The Breaking of the Fellowship", vec!["music", "sad"]),
                Music::new("Lothlórien", vec!["music"]),
                Music::new("A Hobbit's Tale", vec!["music", "heroic"]),
                Music::new("Into the West", vec!["music"]),
                Music::new("Fire", vec!["sound"]),
                Music::new("Rain", vec!["sound"]),
                Music::new("Wind", vec!["sound"]),
            ],
        }
    }

    fn open_project(&mut self) {
        println!("Open project")
    }

    fn save_project(&self) {
        println!("Save project")
    }

    fn search(&mut self) {
        println!("Search for {}", self.search_pattern)
    }

    fn send_source_to_player(source: &Music) {
        println!("Send to player {}", source.title)
    }

    fn send_source_to_map(source: &Music) {
        println!("Send to map {}", source.title)
    }

    pub fn update(&mut self, _ctx: &egui::Context, ui: &mut Ui) {
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            if ui.button("🗁".to_string()).clicked() {
                self.open_project()
            };
            if ui.button("💾".to_string()).clicked() {
                self.save_project()
            };
            ui.vertical_centered(|ui| {
                ui.heading(&self.title);
            });
        });

        ui.add_space(10.0);
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("🔎".to_string()).clicked() {
                self.search();
            };
            let search =
                egui::TextEdit::singleline(&mut self.search_pattern).hint_text("Название или тег");
            ui.add(search);
        });

        ui.add_space(10.0);
        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show(ui, |ui| {
                for source in &self.music {
                    ui.horizontal(|ui| {
                        ui.label(&source.title);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("+".to_string()).clicked() {
                                StoreWidget::send_source_to_map(source);
                            }
                            if ui.button("♫".to_string()).clicked() {
                                StoreWidget::send_source_to_player(source);
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
            });
    }
}
