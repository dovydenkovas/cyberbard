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
use std::env;
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

use crate::application::Application;
use crate::audio::audio::Audio;
use crate::gui::events::Event;
use crate::map::Map;
use crate::player::Player;
use crate::storage::storage::Storage;
use crate::storage::stream::Stream;

use super::events::Events;
use super::map::MapWidget;
use super::player::PlayerWidget;
use super::settings::SettingsWidget;
use super::storage::StorageWidget;

/// Describe Cyberbard main window.
/// Create and update all widgets and connect to core application.
pub struct ApplicationImp {
    application: Application,
    events: VecDeque<Event>,
    storage_widget: StorageWidget,
    map_widget: MapWidget,
    player_widget: PlayerWidget,
    settings_widget: SettingsWidget,
}

impl ApplicationImp {
    /// Create main window struct
    pub fn new(application: Application) -> ApplicationImp {
        let storage = application.get_storage();
        let map = application.get_map();
        let player = application.get_player();
        let composition = application.get_selected_composition();
        ApplicationImp {
            application,
            events: VecDeque::new(),
            storage_widget: StorageWidget::new(storage),
            map_widget: MapWidget::new(map),
            player_widget: PlayerWidget::new(player),
            settings_widget: SettingsWidget::new(composition),
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
                    self.settings_widget.insert_audio(audio);
                }
                Event::PlayerPlay => self.application.player_play(),
                Event::PlayerPause => self.application.player_pause(),
                Event::PlayerStop => self.application.player_stop(),
                Event::PlayerSetVolume { volume } => self.application.player_set_volume(volume),
                Event::Select { audio } => {
                    self.application.set_selected_composition(Some(audio));
                    self.settings_widget.sync_with_application();
                }
                Event::MapNewComposition => {
                    self.application.map_add_composition();
                }
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
                self.settings_widget.update(ctx, ui, &mut self.events);
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
