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

use std::env;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

use crate::audio::audio::Audio;
use crate::map::Map;
use crate::player::Player;
use crate::storage::storage::Storage;
use crate::storage::stream::Stream;

use super::map::MapWidget;
use super::player::PlayerWidget;
use super::settings::SettingsWidget;
use super::storage::StorageWidget;

/// Describe Cyberbard main window.
/// Create and update all widgets and connect to core application.
pub struct Application {
    storage_widget: StorageWidget,
    map_widget: MapWidget,
    player_widget: PlayerWidget,
    settings_widget: SettingsWidget,
}

impl Application {
    /// Create main window struct
    pub fn new(
        storage: Arc<Mutex<dyn Storage>>,
        map: Arc<Mutex<Map>>,
        player: Arc<Mutex<Player>>,
    ) -> Application {
        let (storage2player_tx, storage2player_rx): (
            Sender<(String, Stream)>,
            Receiver<(String, Stream)>,
        ) = mpsc::channel();

        let (map2settings_tx, map2settings_rx): (
            Sender<Arc<Mutex<dyn Audio>>>,
            Receiver<Arc<Mutex<dyn Audio>>>,
        ) = mpsc::channel();

        let (storage2settings_tx, storage2settings_rx): (
            Sender<Box<dyn Audio>>,
            Receiver<Box<dyn Audio>>,
        ) = mpsc::channel();

        let (map2player_tx, map2player_rx): (Sender<(String, Stream)>, Receiver<(String, Stream)>) =
            mpsc::channel();

        Application {
            storage_widget: StorageWidget::new(storage, storage2player_tx, storage2settings_tx),
            map_widget: MapWidget::new(map, map2settings_tx, map2player_tx),
            player_widget: PlayerWidget::new(player, storage2player_rx, map2player_rx),
            settings_widget: SettingsWidget::new(map2settings_rx, storage2settings_rx),
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("Storage")
            .resizable(true)
            .default_width(300.0)
            .width_range(200.0..=400.0)
            .show(ctx, |ui| self.storage_widget.update(ctx, ui));

        egui::SidePanel::right("PlayerAndSettings")
            .resizable(true)
            .default_width(300.0)
            .min_width(160.0)
            .show(ctx, |ui| {
                self.player_widget.update(ctx, ui);
                ui.separator();
                self.settings_widget.update(ctx, ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.map_widget.update(ctx, ui);
        });
    }
}

pub fn run_gui(storage: Arc<Mutex<dyn Storage>>, map: Arc<Mutex<Map>>, player: Arc<Mutex<Player>>) {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).as_str(),
        options,
        Box::new(|_cc| Ok(Box::new(Application::new(storage, map, player)))),
    );
}
