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

use std::sync::{Arc, Mutex, mpsc::Receiver};

use egui::{Ui, warn_if_debug_build};

use crate::audio::audio::Audio;

pub struct SettingsWidget {
    title: String,
    audio: Option<Arc<Mutex<dyn Audio>>>,
    map2settings_rx: Receiver<Arc<Mutex<dyn Audio>>>,
    storage2settings_rx: Receiver<Box<dyn Audio>>,
}
impl SettingsWidget {
    pub fn new(
        map2settings_rx: Receiver<Arc<Mutex<dyn Audio>>>,
        storage2settings_rx: Receiver<Box<dyn Audio>>,
    ) -> SettingsWidget {
        SettingsWidget {
            title: "".to_string(),
            audio: None,
            map2settings_rx,
            storage2settings_rx,
        }
    }

    pub fn update(&mut self, _ctx: &egui::Context, ui: &mut Ui) {
        match self.map2settings_rx.try_recv() {
            Ok(audio) => {
                self.title = audio.lock().unwrap().get_title();
                self.audio = Some(audio);
            }
            Err(_) => (),
        }

        match self.storage2settings_rx.try_recv() {
            Ok(audio) => {
                let _ = self
                    .audio
                    .as_mut()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .insert_audio(0, audio);
            }
            Err(_) => (),
        }

        if self.audio.is_none() {
            return;
        }

        ui.vertical_centered(|ui| {
            ui.heading(&self.title);
        });

        // todo
        // ui.horizontal(|ui| {
        //     let mut scalar = 0.0;
        //     ui.label("Громкость");
        //     ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        //         ui.add(egui::Slider::new(&mut scalar, 0.0..=100.0).show_value(false));
        //     });
        // });

        ui.add_space(20.0);
        ui.vertical_centered(|ui| {
            ui.label("Состав композиции");
            ui.add_space(10.0);
        });

        let n = self.audio.clone().unwrap().lock().unwrap().audio_count();
        for i in 0..n {
            let audio = self
                .audio
                .clone()
                .unwrap()
                .lock()
                .unwrap()
                .get_audio(i)
                .unwrap();

            ui.horizontal(|ui| {
                let mut scalar = 0.0;
                ui.label(audio.get_title());

                // ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                //     ui.small("🗙");
                //     ui.button("🔁");
                //     ui.add(egui::Slider::new(&mut scalar, 0.0..=100.0).show_value(false));
                // });
            });
            ui.add_space(5.0);
        }
    }
}
