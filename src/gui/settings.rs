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

use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex, mpsc::Receiver},
};

use egui::{Ui, warn_if_debug_build};

use crate::{
    audio::{audio::Audio, composition},
    gui::events::Events,
};

pub struct SettingsWidget {
    title: String,
    composition: Rc<RefCell<Option<Rc<RefCell<dyn Audio>>>>>,
}
impl SettingsWidget {
    pub fn new(composition: Rc<RefCell<Option<Rc<RefCell<dyn Audio>>>>>) -> SettingsWidget {
        SettingsWidget {
            title: "".to_string(),
            composition,
        }
    }

    pub fn sync_with_application(&mut self) {
        if let Some(comp) = self.composition.borrow_mut().as_ref() {
            self.title = comp.borrow().get_title();
        }
    }

    pub fn insert_audio(&mut self, audio: Rc<RefCell<dyn Audio>>) {
        if let Some(compostion) = self.composition.borrow_mut().as_ref() {
            compostion.borrow_mut().push_audio(audio).unwrap();
        }
    }

    pub fn update(&mut self, _ctx: &egui::Context, ui: &mut Ui, events: &mut Events) {
        // match self.storage2settings_rx.try_recv() {
        //     Ok(audio) => {
        //         let _ = self
        //             .audio
        //             .as_mut()
        //             .unwrap()
        //             .lock()
        //             .unwrap()
        //             .insert_audio(0, audio);
        //     }
        //     Err(_) => (),
        // }

        if let Some(composition) = self.composition.borrow_mut().as_ref() {
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

            let n = composition.borrow().audio_count();
            for i in 0..n {
                let audio = composition.borrow().get_audio(i).unwrap();

                ui.horizontal(|ui| {
                    let mut scalar = 0.0;
                    ui.label(audio.borrow().get_title());

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
}
