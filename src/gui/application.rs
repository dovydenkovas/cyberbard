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

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::{env, thread};

use eframe::NativeOptions;
use egui::{Style, ViewportBuilder, Visuals};
use rfd::MessageDialog;

use crate::application::Application;
use crate::colors;
use crate::gui::events::Event;
use crate::settings::Settings;

use super::composition::CompositionWidget;
use super::map::MapWidget;
use super::player::PlayerWidget;
use super::storage::StorageWidget;

/// Describe Cyberbard main window.
/// Create and update all widgets and connect to core application.
pub struct ApplicationImp {
    application: Application,
    events: VecDeque<Event>,
    storage_widget: StorageWidget,
    map_widget: MapWidget,
    player_widget: PlayerWidget,
    composition_widget: CompositionWidget,
    settings: Rc<RefCell<Settings>>,
}

impl ApplicationImp {
    /// Create main window struct
    pub fn new(application: Application, settings: Rc<RefCell<Settings>>) -> ApplicationImp {
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
            composition_widget: CompositionWidget::new(composition),
            settings,
        }
    }

    fn handle_events(&mut self, ctx: &egui::Context) {
        while let Some(event) = self.events.pop_front() {
            match event {
                Event::SetupStorage { credentials } => {
                    // TODO: show error message
                    if self.application.setup_storage(credentials).is_ok() {
                        let storage = self.application.get_storage();
                        let map = self.application.get_root_map();
                        let player = self.application.get_player();
                        let composition = self.application.get_selected_composition();
                        self.storage_widget = StorageWidget::new(storage);
                        self.map_widget = MapWidget::new(map);
                        self.player_widget = PlayerWidget::new(player);
                        self.composition_widget = CompositionWidget::new(composition);
                    }
                }
                Event::Play { audio } => {
                    self.player_widget.play(&audio);
                    self.application.player_set_audio(audio);
                    self.application.player_play();
                }
                Event::AddAudioToComposition { audio } => {
                    self.composition_widget.insert_audio(Rc::clone(&audio));
                    self.application.player_sync();
                }
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
                    composition_index,
                    index,
                } => self
                    .application
                    .player_set_track_volume(volume, composition_index, index),

                Event::Select { audio } => {
                    self.application.set_selected_composition(Some(audio));
                    self.composition_widget.sync_with_application();
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
                Event::ToggleTheme => {
                    let is_dark = self.settings.borrow().dark_theme;
                    if is_dark {
                        colors::set_light();
                        ctx.set_visuals(egui::Visuals::light());
                        self.settings.borrow_mut().dark_theme = false;
                    } else {
                        colors::set_dark();
                        ctx.set_visuals(egui::Visuals::dark());
                        self.settings.borrow_mut().dark_theme = true;
                    }
                }
            }
        }
    }
}

impl eframe::App for ApplicationImp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_events(ctx);

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
                self.composition_widget.update(ctx, ui, &mut self.events);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.map_widget.update(ctx, ui, &mut self.events);
        });
    }
}

pub fn run_gui(application: crate::application::Application, settings: Rc<RefCell<Settings>>) {
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size(&settings.borrow().default_size),
        ..Default::default()
    };

    let _ = eframe::run_native(
        format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).as_str(),
        options,
        Box::new(|cc| {
            let visuals = if settings.borrow().dark_theme {
                Visuals::dark()
            } else {
                Visuals::light()
            };

            let style = Style {
                visuals: visuals,
                ..Style::default()
            };
            cc.egui_ctx.set_style(style);
            Ok(Box::new(ApplicationImp::new(application, settings)))
        }),
    );
}
