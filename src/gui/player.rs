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

use std::{cell::RefCell, rc::Rc, time::Duration};

use egui::Ui;

use crate::{
    audio::audio::Audio,
    gui::events::{Event, Events},
    player::Player,
};

pub struct PlayerWidget {
    title: String,
    volume: f32,
    is_pause: bool,
    player: Rc<RefCell<Player>>,
}

impl PlayerWidget {
    pub fn new(player: Rc<RefCell<Player>>) -> PlayerWidget {
        let widget = PlayerWidget {
            title: "".to_string(),
            volume: 1.0,
            is_pause: true,
            player,
        };

        widget
    }

    fn toggle_pause(&mut self, events: &mut Events) {
        self.is_pause = !self.is_pause;
        if self.is_pause {
            events.push_back(Event::PlayerPause)
        } else {
            events.push_back(Event::PlayerPlay)
        }
    }

    fn stop(&mut self, events: &mut Events) {
        events.push_back(Event::PlayerStop);
        self.is_pause = true;
    }

    pub fn play(&mut self, audio: &Rc<RefCell<dyn Audio>>) {
        self.title = audio.borrow().get_title();
        self.is_pause = false;
    }

    fn set_volume(&mut self, events: &mut Events) {
        events.push_back(Event::PlayerSetVolume {
            volume: self.volume,
        });
    }

    pub fn update(&mut self, ctx: &egui::Context, ui: &mut Ui, events: &mut Events) {
        ui.add_space(20.0);
        ui.vertical_centered(|ui| {
            ui.heading(&self.title);
        });

        ui.add_space(20.0);
        let progress_bar =
            egui::ProgressBar::new(self.player.borrow().get_position()).desired_height(4.0);
        ui.add(progress_bar);
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            let pause_char = if self.is_pause { "▶" } else { "⏸" };

            if ui.button(pause_char).clicked() {
                self.toggle_pause(events)
            }

            if ui.button("⏹").clicked() {
                self.stop(events);
            }

            if ui
                .add(egui::Slider::new(&mut self.volume, 0.0..=1.0).show_value(false))
                .changed()
            {
                self.set_volume(events);
            };
        });
        ui.add_space(10.0);

        if !self.is_pause {
            ctx.request_repaint_after(Duration::from_millis(50));
        }
    }
}
