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
    current_playlist: Option<String>,
}

impl PlaylistWidget {
    pub fn new(composition: Rc<RefCell<Option<Audio>>>) -> PlaylistWidget {
        PlaylistWidget {
            title: EditableHeader::new("".to_string()),
            composition,
            current_playlist: None,
        }
    }

    pub fn sync_with_application(&mut self) {
        if let Some(comp) = self.composition.borrow().as_ref() {
            self.title.set_text(comp.borrow().get_title());
            self.current_playlist = None;
        }
    }

    pub fn insert_audio(&mut self, audio: Audio) {
        if let Some(composition) = self.composition.borrow_mut().as_ref() {
            let playlist = if let Some(playlist) = &self.current_playlist {
                playlist.clone()
            } else if let Some(playlist) = composition.borrow().playlists().unwrap().get(0) {
                playlist.clone()
            } else {
                let playlist = generate_playlist_name(Vec::new());
                composition.borrow_mut().push_playlist(&playlist).unwrap();
                playlist
            };

            composition
                .borrow_mut()
                .push_audio(&playlist, audio)
                .unwrap();
        }
    }

    pub fn update(&mut self, _ctx: &egui::Context, ui: &mut Ui, events: &mut Events) {
        if self.composition.borrow().is_none() {
            return;
        }

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
        let playlists = if let Some(v) = self.composition.borrow().as_ref() {
            v.borrow().playlists().unwrap()
        } else {
            Vec::new()
        };

        for mut playlist in playlists {
            let mut remove_elements = vec![];
            self.render_playlist(ui, events, &mut remove_elements, &mut playlist);

            for element in remove_elements {
                self.remove_composition(&playlist, element);
                sync_with_player(events, self.composition.borrow_mut().as_mut().unwrap());
            }
        }

        ui.vertical_centered(|ui| {
            if ui.button("+").clicked() {
                let playlist = &generate_playlist_name(
                    self.composition
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .playlists()
                        .unwrap(),
                );

                self.composition
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .push_playlist(playlist)
                    .unwrap();
            }
        });
    }

    fn render_playlist(
        &mut self,
        ui: &mut Ui,
        events: &mut Events,
        remove_elements: &mut Vec<usize>,
        playlist: &mut String,
    ) {
        if self.composition.borrow().as_ref().is_none() {
            return;
        }

        ui.horizontal(|ui| {
            let mut title = playlist.clone();
            let title_edit = ui.add(
                if self.current_playlist.is_some()
                    && &title == self.current_playlist.as_ref().unwrap()
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
                    .rename_playlist(playlist, &title);
                *playlist = title;
            }

            if title_edit.has_focus() {
                self.current_playlist = Some(playlist.clone());
            }

            if ui.label("🗙").clicked() {
                self.composition
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .remove_playlist(playlist);
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
            .audio_count(&playlist);

        for i in 0..n {
            // TODO: set labels clickable to select a track in the playlist.
            let audio: Audio = self
                .composition
                .borrow()
                .as_ref()
                .unwrap()
                .borrow()
                .get_audio(&playlist, i)
                .unwrap();

            ui.horizontal(|ui| {
                ui.label(audio.borrow().get_title());

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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
                            playlist_index: self
                                .composition
                                .borrow()
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .playlists()
                                .unwrap()
                                .iter()
                                .position(|s| &s == &playlist)
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

    fn remove_composition(&mut self, playlist: &String, index: usize) {
        let _ = self
            .composition
            .borrow()
            .as_ref()
            .unwrap()
            .borrow_mut()
            .remove_audio(playlist, index);
    }
}

fn sync_with_player(
    events: &mut std::collections::VecDeque<super::events::Event>,
    _composition: &Audio,
) {
    events.push_back(super::events::Event::PlayerSync);
}

fn generate_playlist_name(names: Vec<String>) -> String {
    let mut i: usize = 1;
    while i < 100_000 {
        let name = format!("Playlist {i}");
        if !names.contains(&name) {
            return name;
        }
        i += 1;
    }
    "The last name".to_string()
}
