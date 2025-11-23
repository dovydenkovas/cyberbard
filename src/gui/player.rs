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

use egui::Ui;

use crate::{player::Player, storage::stream::Stream};

pub struct PlayerWidget {
    title: String,
    progress: f32,
    volume: f32,
    is_pause: bool,
    is_looped: bool,
    player: Arc<Mutex<Player>>,
    storage2player_rx: Receiver<(String, Stream)>,
}

impl PlayerWidget {
    pub fn new(
        player: Arc<Mutex<Player>>,
        storage2player_rx: Receiver<(String, Stream)>,
    ) -> PlayerWidget {
        PlayerWidget {
            title: "The Shire".to_string(),
            progress: 0.6,
            volume: 100.0,
            is_pause: true,
            is_looped: false,
            player,
            storage2player_rx,
        }
    }

    fn toggle_pause(&mut self) {
        self.is_pause = !self.is_pause;
        if self.is_pause {
            self.player.lock().unwrap().pause();
        } else {
            self.player.lock().unwrap().play();
        }
    }

    fn stop(&mut self) {
        self.player.lock().unwrap().stop();
        self.is_pause = true;
    }

    fn toggle_looped(&mut self) {
        self.is_looped = !self.is_looped;
    }

    fn check_events(&mut self) {
        match self.storage2player_rx.try_recv() {
            Ok((title, stream)) => {
                self.title = title;
                self.player.lock().unwrap().set_stream(stream);
                self.player.lock().unwrap().play();
                self.is_pause = false;
            }
            Err(_) => (),
        }
    }

    pub fn update(&mut self, _ctx: &egui::Context, ui: &mut Ui) {
        self.check_events();

        ui.add_space(20.0);
        ui.vertical_centered(|ui| {
            ui.heading(&self.title);
        });

        ui.add_space(20.0);
        let progress_bar = egui::ProgressBar::new(self.progress).desired_height(4.0);
        ui.add(progress_bar);
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            let pause_char = if self.is_pause { "▶" } else { "⏸" };

            if ui.button(pause_char).clicked() {
                self.toggle_pause()
            }

            if ui.button("⏹").clicked() {
                self.stop();
            }

            let loop_btn = egui::Button::new("🔁").selected(self.is_looped);
            if ui.add(loop_btn).clicked() {
                self.toggle_looped();
            }

            ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.0).show_value(false));
        });
        ui.add_space(10.0);
    }
}
