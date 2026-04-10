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

use egui::{Color32, Label, RichText, Sense, Slider, TextEdit, Ui, UiBuilder};

use crate::{
    application::Application,
    audio::{Audio, playlist::Playlist},
    gui::events::{Event, Events},
};

pub struct PlaylistWidget {
    current_thread: Option<String>,
    application: Rc<RefCell<Application>>,
}

impl PlaylistWidget {
    pub fn new(application: Rc<RefCell<Application>>) -> PlaylistWidget {
        PlaylistWidget {
            current_thread: None,
            application,
        }
    }

    pub fn sync_with_application(&mut self) {
        if self.application.borrow().get_selected_playlist().is_some() {
            self.current_thread = None;
        }
    }

    pub fn insert_audio(&mut self, audio: Audio) {
        if let Some(playlist) = self.application.borrow().get_selected_playlist()
            && let Audio::Playlist(ref mut playlist) = *playlist.borrow_mut()
        {
            let thread = if let Some(thread) = &self.current_thread {
                thread.clone()
            } else if let Some(thread) = playlist.threads().unwrap().first() {
                thread.clone()
            } else {
                let thread = generate_thread_name(Vec::new());
                playlist.push_thread(&thread).unwrap();
                thread
            };

            playlist.push_audio(&thread, audio).unwrap();
        }
    }

    pub fn update(&mut self, _ctx: &egui::Context, ui: &mut Ui, events: &mut Events) {
        let comp = self.application.borrow().get_selected_playlist();
        if let Some(playlist) = comp {
            let builder = UiBuilder::new().sense(Sense::click());
            if ui
                .scope_builder(builder, |ui| {
                    egui::ScrollArea::vertical()
                        .vscroll(true)
                        .auto_shrink(false)
                        .show(ui, |ui| {
                            let mut title = playlist.borrow().get_title();
                            ui.vertical_centered(|ui| {
                                if ui
                                    .add(
                                        TextEdit::singleline(&mut title)
                                            .horizontal_align(egui::Align::Center)
                                            .text_color(ui.visuals().strong_text_color())
                                            .background_color(ui.visuals().panel_fill),
                                    )
                                    .changed()
                                {
                                    playlist.borrow_mut().set_title(title);
                                    sync_with_player(events);
                                }
                            });

                            ui.add_space(20.0);

                            ui.horizontal(|ui| {
                                ui.label(t!("volume"));
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        let mut total_volume = playlist.borrow().get_volume();
                                        if ui
                                            .add(
                                                Slider::new(&mut total_volume, 0.0..=1.0)
                                                    .show_value(false),
                                            )
                                            .changed()
                                        {
                                            playlist.borrow_mut().set_volume(total_volume);
                                            sync_with_player(events);
                                        }
                                    },
                                );
                            });
                            ui.add_space(25.0);

                            let current_playing = if self
                                .application
                                .borrow()
                                .is_playing(Some(Rc::clone(&playlist)))
                            {
                                Some(
                                    self.application
                                        .borrow()
                                        .get_player()
                                        .borrow()
                                        .get_current_playing_indexes(),
                                )
                            } else {
                                None
                            };

                            let mut guard = playlist.borrow_mut();
                            if let Audio::Playlist(ref mut playlist) = *guard {
                                let threads = playlist.threads().unwrap();

                                for mut thread in threads {
                                    let mut remove_elements = vec![];
                                    let index = playlist.index_of_thread(&thread);

                                    self.render_thread(
                                        ui,
                                        events,
                                        &mut remove_elements,
                                        &mut thread,
                                        playlist,
                                        current_playing.as_ref().unwrap_or(&vec![]).get(index),
                                    );

                                    for element in remove_elements {
                                        let _ = playlist.remove_audio(&thread, element);
                                        sync_with_player(events);
                                        if playlist.is_thread_empty(&thread) {
                                            playlist.remove_thread(&thread);
                                        }
                                    }
                                }

                                ui.vertical_centered(|ui| {
                                    let threads = playlist.threads().unwrap();
                                    let last_thread: Option<&String> = threads.last();
                                    if ui.button("+").clicked()
                                        && (last_thread.is_none()
                                            || !playlist.is_thread_empty(last_thread.unwrap()))
                                    {
                                        let thread =
                                            generate_thread_name(playlist.threads().unwrap());

                                        playlist.push_thread(&thread).unwrap();
                                        self.current_thread = Some(thread);
                                    }
                                });
                            }
                        });
                })
                .response
                .clicked()
            {
                self.current_thread = None;
            }
        }
    }

    fn render_thread(
        &mut self,
        ui: &mut Ui,
        events: &mut Events,
        remove_elements: &mut Vec<usize>,
        thread: &mut String,
        playlist: &mut Playlist,
        current_playing: Option<&usize>,
    ) {
        let mut title = thread.clone();
        let color =
            if self.current_thread.is_some() && &title == self.current_thread.as_ref().unwrap() {
                ui.visuals().disable(ui.visuals().selection.bg_fill)
            } else {
                ui.visuals()
                    .disable(ui.visuals().widgets.inactive.weak_bg_fill)
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
                    playlist.rename_thread(thread, &title);
                    *thread = title;
                }

                if title_edit.has_focus() {
                    self.current_thread = Some(thread.clone());
                }

                if ui.label("🗙").clicked() {
                    playlist.remove_thread(thread);
                }
            });

            ui.add_space(5.0);
            let n = playlist.audio_count(thread);

            for i in 0..n {
                // TODO: set labels clickable to select a track in the thread.
                let audio = playlist.get_audio(thread, i).unwrap();

                ui.horizontal(|ui| {
                    let text = if current_playing.is_some() && &i == current_playing.unwrap() {
                        RichText::new(audio.borrow().get_title()).strong()
                    } else {
                        RichText::new(audio.borrow().get_title())
                    };

                    if ui.label(text).clicked() && current_playing.is_some() {
                        self.application
                            .borrow_mut()
                            .get_player()
                            .borrow_mut()
                            .goto_track(playlist.index_of_thread(thread), i);
                    };

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
                                playlist_index: playlist
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

fn sync_with_player(events: &mut std::collections::VecDeque<super::events::Event>) {
    events.push_back(super::events::Event::PlayerSync);
}

fn generate_thread_name(names: Vec<String>) -> String {
    let mut i: usize = 1;
    while i < 100_000 {
        let name = format!("{} {i}", t!("thread"));
        if !names.contains(&name) {
            return name;
        }
        i += 1;
    }
    "The last name".to_string()
}
