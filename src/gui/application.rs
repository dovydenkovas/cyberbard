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

use std::collections::VecDeque;
use std::rc::Rc;
use std::{env, thread};

use rfd::MessageDialog;

use crate::application::Application;
use crate::gui::events::Event;

use super::map::MapWidget;
use super::player::PlayerWidget;
use super::playlist::PlaylistWidget;
use super::storage::StorageWidget;

/// Describe Cyberbard main window.
/// Create and update all widgets and connect to core application.
pub struct ApplicationImp {
    application: Application,
    events: VecDeque<Event>,
    storage_widget: StorageWidget,
    map_widget: MapWidget,
    player_widget: PlayerWidget,
    playlist_widget: PlaylistWidget,
}

impl ApplicationImp {
    /// Create main window struct
    pub fn new(application: Application) -> ApplicationImp {
        let storage = application.get_storage();
        let map = application.get_root_map();
        let player = application.get_player();
        let composition = application.get_selected_composition();
        ApplicationImp {
            application,
            events: VecDeque::new(),
            storage_widget: StorageWidget::new(storage),
            map_widget: MapWidget::new(map),
            player_widget: PlayerWidget::new(player),
            playlist_widget: PlaylistWidget::new(composition),
        }
    }

    fn handle_events(&mut self) {
        while let Some(event) = self.events.pop_front() {
            match event {
                Event::SetupStorage { credentials } => {
                    self.application.setup_storage(credentials);
                    self.storage_widget.sync_with_storage();
                }
                Event::Play { audio } => {
                    self.player_widget.play(&audio);
                    self.application.player_set_audio(audio);
                    self.application.player_play();
                }
                Event::AddAudioToComposition { audio } => {
                    self.playlist_widget.insert_audio(Rc::clone(&audio));
                    self.application.player_sync();
                }
                Event::PlayerPlay => self.application.player_play(),
                Event::PlayerPause => self.application.player_pause(),
                Event::PlayerStop => self.application.player_stop(),
                Event::PlayerSync => {
                    self.application.player_sync();
                    self.player_widget.play(
                        self.application
                            .get_current_playing()
                            .borrow()
                            .as_ref()
                            .unwrap(),
                    );
                }
                Event::PlayerSetVolume { volume } => self.application.player_set_volume(volume),
                Event::PlayerSetTrackVolume {
                    volume,
                    playlist_index,
                    index,
                } => self
                    .application
                    .player_set_track_volume(volume, playlist_index, index),

                Event::Select { audio } => {
                    self.application.set_selected_composition(Some(audio));
                    self.playlist_widget.sync_with_application();
                }
                Event::SaveProject { path } => match self.application.save_project(path) {
                    Ok(_) => (),
                    Err(err) => {
                        let err = err.to_string();
                        thread::spawn(move || {
                            MessageDialog::new()
                                .set_title("Ошибка сохранения файла")
                                .set_description(err)
                                .set_level(rfd::MessageLevel::Error)
                                .show();
                        });
                    }
                },
            }
        }
    }
}

impl eframe::App for ApplicationImp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_events();

        egui::SidePanel::left("Storage")
            .resizable(true)
            .default_width(300.0)
            .width_range(200.0..=400.0)
            .show(ctx, |ui| {
                self.storage_widget.update(ctx, ui, &mut self.events)
            });

        egui::SidePanel::right("PlayerAndSettings")
            .resizable(true)
            .default_width(300.0)
            .min_width(160.0)
            .show(ctx, |ui| {
                self.player_widget.update(ctx, ui, &mut self.events);
                ui.separator();
                self.playlist_widget.update(ctx, ui, &mut self.events);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.map_widget.update(ctx, ui, &mut self.events);
        });
    }
}

pub fn run_gui(application: crate::application::Application) {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).as_str(),
        options,
        Box::new(|_cc| Ok(Box::new(ApplicationImp::new(application)))),
    );
}
