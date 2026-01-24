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

use egui::{Color32, Label, Sense, Slider, TextEdit, Ui};

use crate::{
    application::Application,
    audio::Audio,
    gui::{
        events::{Event, Events},
    },
};

pub struct CompositionWidget {
    // composition: Rc<RefCell<Option<Audio>>>,
    current_thread: Option<String>,
    application: Rc<RefCell<Application>>,
}

impl CompositionWidget {
    pub fn new(application: Rc<RefCell<Application>>) -> CompositionWidget {
        CompositionWidget {
            current_thread: None,
            application,
        }
    }

    pub fn sync_with_application(&mut self) {
        if let Some(_) = self
            .application
            .borrow()
            .get_selected_composition()
            .borrow_mut()
            .as_ref()
        {
            self.current_thread = None;
        }
    }

    pub fn insert_audio(&mut self, audio: Audio) {
        if let Some(composition) = self
            .application
            .borrow()
            .get_selected_composition()
            .borrow_mut()
            .as_ref()
        {
            let thread = if let Some(thread) = &self.current_thread {
                thread.clone()
            } else if let Some(thread) = composition.borrow().threads().unwrap().first() {
                thread.clone()
            } else {
                let thread = generate_thread_name(Vec::new());
                composition.borrow_mut().push_thread(&thread).unwrap();
                thread
            };

            composition.borrow_mut().push_audio(&thread, audio).unwrap();
        }
    }

    pub fn update(&mut self, _ctx: &egui::Context, ui: &mut Ui, events: &mut Events) {
        let comp = self.application.borrow().get_selected_composition();
        if let Some(composition) = comp.borrow_mut().as_ref() {
            egui::ScrollArea::vertical()
                .vscroll(true)
                .auto_shrink(false)
                .show(ui, |ui| {
                    let mut title = composition.borrow().get_title();
                    ui.vertical_centered(|ui| {
                        if ui.add(
                            TextEdit::singleline(&mut title)
                                .horizontal_align(egui::Align::Center)
                                .text_color(ui.visuals().strong_text_color())
                                .background_color(ui.visuals().panel_fill),
                        ).changed() {
                            composition.borrow_mut().set_title(title);
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
                    ui.add_space(25.0);
                    let threads = composition.borrow().threads().unwrap();

                    for mut thread in threads {
                        let mut remove_elements = vec![];
                        self.render_thread(
                            ui,
                            events,
                            &mut remove_elements,
                            &mut thread,
                            &composition,
                        );

                        for element in remove_elements {
                            let _ = composition.borrow_mut().remove_audio(&thread, element);
                            sync_with_player(events, composition);
                        }
                    }

                    ui.vertical_centered(|ui| {
                        if ui.button("+").clicked() {
                            let thread =
                                &generate_thread_name(composition.borrow().threads().unwrap());

                            composition.borrow_mut().push_thread(thread).unwrap();
                        }
                    });
                });
        }
    }

    fn render_thread(
        &mut self,
        ui: &mut Ui,
        events: &mut Events,
        remove_elements: &mut Vec<usize>,
        thread: &mut String,
        composition: &Audio,
    ) {
        let mut title = thread.clone();
        let color =
            if self.current_thread.is_some() && &title == self.current_thread.as_ref().unwrap() {
                ui.visuals().disable(ui.visuals().selection.bg_fill)
            } else {
                ui.visuals().disable(ui.visuals().widgets.inactive.weak_bg_fill)
            };

        let frame = egui::Frame::new()
            .fill(color)
            .corner_radius(10)
            .inner_margin(egui::Margin::same(5));
        frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                let title_edit = ui.add(
                    TextEdit::singleline(&mut title)
                        .horizontal_align(egui::Align::Center)
                        .text_color(ui.visuals().strong_text_color())
                        .background_color(Color32::TRANSPARENT),
                );
                if title_edit.changed() {
                    composition.borrow_mut().rename_thread(thread, &title);
                    *thread = title;
                }

                if title_edit.has_focus() {
                    self.current_thread = Some(thread.clone());
                }

                if ui.label("🗙").clicked() {
                    composition.borrow_mut().remove_thread(thread);
                }
            });

            ui.add_space(5.0);
            let n = composition.borrow().audio_count(thread);

            for i in 0..n {
                // TODO: set labels clickable to select a track in the thread.
                let audio: Audio = composition.borrow().get_audio(thread, i).unwrap();

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
                                volume,
                                composition_index: composition
                                    .borrow()
                                    .threads()
                                    .unwrap()
                                    .iter()
                                    .position(|s| s == thread)
                                    .unwrap(),
                                index: i,
                            });
                        }
                    });
                });
                ui.add_space(5.0);
            }
        });
        ui.add_space(5.0);
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
