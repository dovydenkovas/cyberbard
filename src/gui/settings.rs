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

use egui::Ui;

pub struct SettingsWidget {
    title: String,
}

impl SettingsWidget {
    pub fn new() -> SettingsWidget {
        SettingsWidget {
            title: "Основная тема".to_string(),
        }
    }

    pub fn update(&mut self, _ctx: &egui::Context, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading(&self.title);
        });

        ui.horizontal(|ui| {
            let mut scalar = 0.0;
            ui.label("Громкость");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add(egui::Slider::new(&mut scalar, 0.0..=100.0).show_value(false));
            });
        });

        ui.horizontal(|ui| {
            let mut scalar = 0.0;
            ui.label("Цикличность");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.button("🔁");
            });
        });

        ui.add_space(20.0);
        ui.vertical_centered(|ui| {
            ui.label("Состав композиции");
            ui.add_space(10.0);
        });

        ui.horizontal(|ui| {
            let mut scalar = 0.0;
            ui.label("The Shire");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.small("🗙");
                ui.button("🔁");
                ui.add(egui::Slider::new(&mut scalar, 0.0..=100.0).show_value(false));
            });
        });
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            let mut scalar = 0.0;
            ui.label("Rain");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.small("🗙");
                ui.button("🔁");
                ui.add(egui::Slider::new(&mut scalar, 0.0..=100.0).show_value(false));
            });
        });
    }
}
