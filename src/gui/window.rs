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

use std::sync::{Arc, Mutex};

use crate::player::Player;

use super::map::MapWidget;
use super::player::PlayerWidget;
use super::settings::SettingsWidget;
use super::storage::StoreWidget;

/// Describe Cyberbard main window.
/// Create and update all widgets and connect to core application.
pub struct Application {
    storage_widget: StoreWidget,
    map_widget: MapWidget,
    player_widget: PlayerWidget,
    settings_widget: SettingsWidget,
}

impl Application {
    /// Create main window struct
    pub fn new(player: Arc<Mutex<Player>>) -> Application {
        Application {
            storage_widget: StoreWidget::new(),
            map_widget: MapWidget::new(),
            player_widget: PlayerWidget::new(player),
            settings_widget: SettingsWidget::new(),
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

pub fn run_gui(player: Arc<Mutex<Player>>) {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Cyberbard",
        options,
        Box::new(|_cc| Ok(Box::new(Application::new(player)))),
    );
}
