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

use egui::{Label, Sense, Ui};

use crate::{
    audio::audio::{Audio, RawAudio},
    gui::{
        events::{Event, Events},
        widgets::{EditableHeader, EditableSlider},
    },
};

enum Action {
    Remove(usize),
    Sync,
}

pub struct PlaylistWidget {
    title: EditableHeader,
    composition: Rc<RefCell<Option<Audio>>>,
    total_volume: EditableSlider,
    volumes: Vec<EditableSlider>,
}

impl PlaylistWidget {
    pub fn new(composition: Rc<RefCell<Option<Audio>>>) -> PlaylistWidget {
        let mut volumes = vec![];
        if let Some(c) = composition.borrow().as_ref() {
            for _ in 0..c.borrow().audio_count() {
                volumes.push(EditableSlider::new());
            }
        }

        PlaylistWidget {
            title: EditableHeader::new("".to_string()),
            composition,
            total_volume: EditableSlider::new(),
            volumes,
        }
    }

    pub fn sync_with_application(&mut self) {
        if let Some(comp) = self.composition.borrow().as_ref() {
            self.title.set_text(comp.borrow().get_title());
            self.total_volume.set_value(comp.borrow().get_volume());
            println!("{:?}", self.total_volume);

            let mut volumes = vec![];
            for i in 0..comp.borrow().audio_count() {
                volumes.push(EditableSlider::from(
                    comp.borrow().get_audio(i).unwrap().borrow().get_volume(),
                ));
            }
            self.volumes = volumes;
        }
    }

    pub fn insert_audio(&mut self, audio: Audio) {
        if let Some(compostion) = self.composition.borrow_mut().as_ref() {
            compostion.borrow_mut().push_audio(audio).unwrap();
            self.volumes.push(EditableSlider::new());
        }
    }

    pub fn update(&mut self, _ctx: &egui::Context, ui: &mut Ui, events: &mut Events) {
        // match self.storage2settings_rx.try_recv() {
        //     Ok(audio) => {
        //         let _ = self
        //             .audio
        //             .as_mut()
        //             .unwrap()
        //             .lock()
        //             .unwrap()
        //             .insert_audio(0, audio);
        //     }
        //     Err(_) => (),
        // }

        let mut actions = vec![];
        if let Some(composition) = self.composition.borrow().as_ref() {
            ui.vertical_centered(|ui| {
                if let Some(new_title) = self.title.update(ui) {
                    composition.borrow_mut().set_title(new_title);
                }
            });

            ui.add_space(20.0);
            ui.horizontal(|ui| {
                ui.label("Громкость");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if let Some(v) = self.total_volume.update(ui) {
                        composition.borrow_mut().set_volume(v);
                        sync_with_player(events, composition);
                    }
                });
            });
        }
        ui.add_space(25.0);
        self.render_playlist(ui, "Музыка", events, &mut actions);
        // self.render_playlist(ui, "Эффекты", composition);

        for action in actions {
            match action {
                Action::Remove(i) => {
                    self.remove_composition(i);
                    sync_with_player(events, self.composition.borrow_mut().as_mut().unwrap());
                }
                Action::Sync => {
                    sync_with_player(events, self.composition.borrow_mut().as_mut().unwrap())
                }
            }
        }
    }

    fn render_playlist(
        &mut self,
        ui: &mut Ui,
        title: &str,
        events: &mut Events,
        actions: &mut Vec<Action>,
    ) {
        if self.composition.borrow().as_ref().is_none() {
            return;
        }

        ui.separator();
        ui.heading(title);
        ui.add_space(20.0);
        let n = self
            .composition
            .borrow()
            .as_ref()
            .unwrap()
            .borrow()
            .audio_count();

        for i in 0..n {
            let audio: Audio = self
                .composition
                .borrow()
                .as_ref()
                .unwrap()
                .borrow()
                .get_audio(i)
                .unwrap();

            ui.horizontal(|ui| {
                ui.label(audio.borrow().get_title());

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .add(Label::new("🗙").sense(Sense::click()).selectable(false))
                        .clicked()
                    {
                        actions.push(Action::Remove(i));
                    }

                    if let Some(v) = self.volumes[i].update(ui) {
                        self.composition
                            .borrow()
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .get_audio(i)
                            .unwrap()
                            .borrow_mut()
                            .set_volume(v);
                        events.push_back(Event::PlayerSetTrackVolume {
                            volume: v,
                            playlist: 0,
                            index: i,
                        });
                    }
                });
            });
            ui.add_space(5.0);
        }
        ui.add_space(20.0);
    }

    fn remove_composition(&mut self, index: usize) {
        let _ = self
            .composition
            .borrow()
            .as_ref()
            .unwrap()
            .borrow_mut()
            .erase_audio(index);
        let _ = self.volumes.remove(index);
    }
}

fn sync_with_player(
    events: &mut std::collections::VecDeque<super::events::Event>,
    composition: &Rc<RefCell<dyn RawAudio + 'static>>,
) {
    events.push_back(super::events::Event::PlayerSync);
}
