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

use egui::{Label, Sense, Slider, TextEdit, Ui};

use crate::{
    audio::audio::Audio,
    gui::{
        events::{Event, Events},
        widgets::EditableHeader,
    },
};

pub struct PlaylistWidget {
    title: EditableHeader,
    composition: Rc<RefCell<Option<Audio>>>,
    current_thread: Option<String>,
}

impl PlaylistWidget {
    pub fn new(composition: Rc<RefCell<Option<Audio>>>) -> PlaylistWidget {
        PlaylistWidget {
            title: EditableHeader::new("".to_string()),
            composition,
            current_thread: None,
        }
    }

    pub fn sync_with_application(&mut self) {
        if let Some(comp) = self.composition.borrow().as_ref() {
            self.title.set_text(comp.borrow().get_title());
            self.current_thread = None;
        }
    }

    pub fn insert_audio(&mut self, audio: Audio) {
        if let Some(composition) = self.composition.borrow_mut().as_ref() {
            let thread = if let Some(thread) = &self.current_thread {
                thread.clone()
            } else if let Some(thread) = composition.borrow().threads().unwrap().get(0) {
                thread.clone()
            } else {
                let thread = generate_thread_name(Vec::new());
                composition.borrow_mut().push_thread(&thread).unwrap();
                thread
            };

            composition
                .borrow_mut()
                .push_audio(&thread, audio)
                .unwrap();
        }
    }

    pub fn update(&mut self, _ctx: &egui::Context, ui: &mut Ui, events: &mut Events) {
        if self.composition.borrow().is_none() {
            return;
        }

        egui::ScrollArea::vertical()
            .vscroll(true)
            .auto_shrink(false)
            .show(ui, |ui| {
                if let Some(composition) = self.composition.borrow().as_ref() {
                    ui.vertical_centered(|ui| {
                        if let Some(new_title) = self.title.update(ui) {
                            composition.borrow_mut().set_title(new_title);
                            sync_with_player(events, composition);
                        }
                    });

                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        ui.label("Громкость");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let mut total_volume = composition.borrow().get_volume();
                            if ui
                                .add(Slider::new(&mut total_volume, 0.0..=1.0).show_value(false))
                                .changed()
                            {
                                composition.borrow_mut().set_volume(total_volume);
                                sync_with_player(events, composition);
                            }
                        });
                    });
                }
                ui.add_space(25.0);
                let threads = if let Some(v) = self.composition.borrow().as_ref() {
                    v.borrow().threads().unwrap()
                } else {
                    Vec::new()
                };

                for mut thread in threads {
                    let mut remove_elements = vec![];
                    self.render_thread(ui, events, &mut remove_elements, &mut thread);

                    for element in remove_elements {
                        self.remove_composition(&thread, element);
                        sync_with_player(events, self.composition.borrow_mut().as_mut().unwrap());
                    }
                }

                ui.vertical_centered(|ui| {
                    if ui.button("+").clicked() {
                        let thread = &generate_thread_name(
                            self.composition
                                .borrow()
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .threads()
                                .unwrap(),
                        );

                        self.composition
                            .borrow()
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .push_thread(thread)
                            .unwrap();
                    }
                });
            });
    }

    fn render_thread(
        &mut self,
        ui: &mut Ui,
        events: &mut Events,
        remove_elements: &mut Vec<usize>,
        thread: &mut String,
    ) {
        if self.composition.borrow().as_ref().is_none() {
            return;
        }

        ui.horizontal(|ui| {
            let mut title = thread.clone();
            let title_edit = ui.add(
                if self.current_thread.is_some()
                    && &title == self.current_thread.as_ref().unwrap()
                {
                    TextEdit::singleline(&mut title).text_color(ui.visuals().strong_text_color())
                } else {
                    TextEdit::singleline(&mut title)
                },
            );
            if title_edit.changed() {
                self.composition
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .rename_thread(thread, &title);
                *thread = title;
            }

            if title_edit.has_focus() {
                self.current_thread = Some(thread.clone());
            }

            if ui.label("🗙").clicked() {
                self.composition
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .remove_thread(thread);
                return;
            }
        });

        ui.add_space(5.0);
        let n = self
            .composition
            .borrow()
            .as_ref()
            .unwrap()
            .borrow()
            .audio_count(&thread);

        for i in 0..n {
            // TODO: set labels clickable to select a track in the thread.
            let audio: Audio = self
                .composition
                .borrow()
                .as_ref()
                .unwrap()
                .borrow()
                .get_audio(&thread, i)
                .unwrap();

            ui.horizontal(|ui| {
                ui.label(audio.borrow().get_title());

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(15.0);
                    if ui
                        .add(Label::new("🗙").sense(Sense::click()).selectable(false))
                        .clicked()
                    {
                        remove_elements.push(i);
                    }

                    let mut volume = audio.borrow().get_volume();
                    if ui
                        .add(Slider::new(&mut volume, 0.0..=1.0).show_value(false))
                        .changed()
                    {
                        audio.borrow_mut().set_volume(volume);
                        events.push_back(Event::PlayerSetTrackVolume {
                            volume: volume,
                            thread_index: self
                                .composition
                                .borrow()
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .threads()
                                .unwrap()
                                .iter()
                                .position(|s| &s == &thread)
                                .unwrap(),
                            index: i,
                        });
                    }
                });
            });
            ui.add_space(5.0);
        }
        ui.separator();
        ui.add_space(20.0);
    }

    fn remove_composition(&mut self, thread: &String, index: usize) {
        let _ = self
            .composition
            .borrow()
            .as_ref()
            .unwrap()
            .borrow_mut()
            .remove_audio(thread, index);
    }
}

fn sync_with_player(
    events: &mut std::collections::VecDeque<super::events::Event>,
    _composition: &Audio,
) {
    events.push_back(super::events::Event::PlayerSync);
}

fn generate_thread_name(names: Vec<String>) -> String {
    let mut i: usize = 1;
    while i < 100_000 {
        let name = format!("Поток {i}");
        if !names.contains(&name) {
            return name;
        }
        i += 1;
    }
    "The last name".to_string()
}
