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

use egui::{Color32, Label, RichText, Sense, Ui};
use rfd::FileDialog;

use crate::{
    audio::{Audio, track::Track},
    gui::{
        events::{Event, Events},
        widgets,
    },
    storage::{Storage, StorageCredentials},
};

pub struct StorageWidget {
    caption: widgets::EditableHeader,
    search_pattern: String,
    shown_music: Vec<usize>,
    storage: Rc<RefCell<Box<dyn Storage>>>,
    edit_track_index: Option<usize>,
}

impl StorageWidget {
    pub fn new(storage: Rc<RefCell<Box<dyn Storage>>>) -> StorageWidget {
        let mut widget = StorageWidget {
            caption: widgets::EditableHeader::new("".to_string()),
            search_pattern: "".to_string(),
            storage,
            shown_music: vec![],
            edit_track_index: None,
        };
        widget.sync_with_storage();
        widget
    }

    pub fn sync_with_storage(&mut self) {
        self.caption.set_text(self.storage.borrow().get_caption());
        self.find();
    }

    fn open_project(&mut self, events: &mut Events) {
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
        let path = FileDialog::new()
            .set_title("Сохраняем проект")
            .add_filter("Теги и карты", &["yaml"])
            .save_file();

        if let Some(path) = path {
            events.push_back(Event::SaveProject {
                path
            });
        }
    }

    fn find(&mut self) {
        let pattern = self.search_pattern.clone();
        self.shown_music = self.storage.borrow().find(pattern);
    }

    fn send_source_to_player(&self, index: usize, events: &mut Events) {
        let audio: Audio = Rc::new(RefCell::new(Box::new(Track::new(
            self.storage.borrow().get(index).unwrap(),
        ))));
        events.push_back(Event::Play { audio });
    }

    fn send_source_to_map(&self, index: usize, events: &mut Events) {
        let source = self.storage.borrow().get(index).unwrap();
        let audio: Audio = Rc::new(RefCell::new(Box::new(Track::new(source))));
        events.push_back(Event::AddAudioToComposition { audio });
    }

    pub fn update(&mut self, ctx: &egui::Context, ui: &mut Ui, events: &mut Events) {
        if let Some(index) = self.edit_track_index {
            self.render_edit_track_dialog(ctx, ui, index, events);
        }

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            if ui.button("🗁".to_string()).clicked() {
                self.open_project(events)
            };
            if ui.button("💾".to_string()).clicked() {
                self.save_project(events)
            };
            ui.vertical_centered(|ui| {
                if let Some(new_caption) = self.caption.update(ui) {
                    self.storage.borrow_mut().set_caption(new_caption);
                }
            });
        });

        ui.add_space(10.0);
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("🔎".to_string());
            let search =
                egui::TextEdit::singleline(&mut self.search_pattern).hint_text("Название или тег");
            if ui.add(search).changed() {
                self.find();
            }
        });

        ui.add_space(10.0);
        let text_style = egui::TextStyle::Body;
        let row_height = ui.text_style_height(&text_style) + 4.0;

        egui::ScrollArea::vertical()
            .vscroll(true)
            .auto_shrink(false)
            .show_rows(ui, row_height, self.shown_music.len(), |ui, range| {
                for i in range {
                    let new_search_pattern = self.render_music(ui, self.shown_music[i], events);
                    if let Some(pattern) = new_search_pattern {
                        self.search_pattern = pattern;
                        self.find();
                        break;
                    }
                }
            });
    }

    /// Display one track. Could return new search pattern
    fn render_music(&mut self, ui: &mut Ui, index: usize, events: &mut Events) -> Option<String> {
        let mut new_search_pattern = None;
        ui.horizontal(|ui| {
            let title_label = Label::new(self.storage.borrow().get(index).unwrap().get_title())
                .sense(Sense::click())
                .selectable(false);
            let ui_label = ui.add(title_label);
            if ui_label.clicked() {
                self.send_source_to_player(index, events);
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(20.0);
                if ui.button("+".to_string()).clicked() {
                    self.send_source_to_map(index, events);
                }

                let mut total_length = 0;
                for tag in self.storage.borrow().get_tags(index) {
                    // TODO: contrasting background and text colors
                    let frame = egui::Frame::new()
                        .fill(Color32::from_hex(&tag.get_color()).unwrap())
                        .corner_radius(5)
                        .inner_margin(egui::Margin::same(2));

                    frame.show(ui, |ui| {
                        let response = if total_length > 20 {
                            ui.add(Label::new("   ").sense(Sense::hover()))
                                .on_hover_text(tag.get_text())
                        } else {
                            total_length += tag.get_text().len();
                            ui.add(Label::new(tag.get_text()))
                        };

                        // Search tag on clicked.
                        if response.clicked() {
                            new_search_pattern = Some(tag.get_text());
                        }

                        if response.secondary_clicked() {
                            self.edit_track_index = Some(index);
                        }
                    });
                }
            });
        });
        ui.add_space(3.0);
        new_search_pattern
    }

    fn render_edit_track_dialog(
        &mut self,
        ctx: &egui::Context,
        _ui: &mut Ui,
        index: usize,
        _events: &mut Events,
    ) {
        egui::Window::new("Настройка тегов")
            .resizable(true)
            .default_size(egui::vec2(300.0, 200.0))
            .show(ctx, |ui| {
                ui.heading(format!(
                    "Теги для {}",
                    self.storage.borrow().get(index).unwrap().get_title()
                ));
                ui.separator();

                let tags = self.storage.borrow().all_tags(index);
                egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .vscroll(true)
                    .show(ui, |ui| {
                        for (tag, mut is_checked) in tags {
                            ui.horizontal(|ui| {
                                // attach unattach tag
                                if ui.checkbox(&mut is_checked, "").changed() {
                                    if is_checked {
                                        self.storage.borrow_mut().attach_tag(index, tag.get_text());
                                    } else {
                                        self.storage
                                            .borrow_mut()
                                            .unattach_tag(index, tag.get_text());
                                    }
                                }

                                // Pick color
                                let color = Color32::from_hex(&tag.get_color()).unwrap();
                                let mut col = [color.r(), color.g(), color.b()];

                                if ui.color_edit_button_srgb(&mut col).changed() {
                                    let color = Color32::from_rgb_additive(col[0], col[1], col[2]);
                                    self.storage.borrow_mut().set_tag_color(
                                        tag.get_text(),
                                        color.to_hex().chars().take(7).collect(),
                                    );
                                }

                                // Change tag text
                                let frame = egui::Frame::new()
                                    .fill(Color32::from_hex(&tag.get_color()).unwrap())
                                    .corner_radius(5)
                                    .inner_margin(egui::Margin::same(2));

                                let mut text = tag.get_text();
                                frame.show(ui, |ui| {
                                    if ui.text_edit_singleline(&mut text).changed() {
                                        self.storage.borrow_mut().rename_tag(tag.get_text(), text);
                                    }
                                });

                                // Remove tag
                                if ui
                                    .label(RichText::new("x".to_string()).color(Color32::RED))
                                    .clicked()
                                {
                                    self.storage.borrow_mut().remove_tag(tag.get_text());
                                }
                                ui.add_space(20.0);
                            });
                        }
                    });

                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    if ui.button("Добавить тег").clicked() {
                        self.storage.borrow_mut().add_tag();
                    }
                });
                ui.add_space(10.0);

                ui.separator();
                ui.vertical_centered_justified(|ui| {
                    if ui.button("Готово").clicked() {
                        self.edit_track_index = None;
                    }
                });
            });
    }
}
