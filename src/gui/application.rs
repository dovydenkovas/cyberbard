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
    application: Rc<RefCell<Application>>,
    events: VecDeque<Event>,
    storage_widget: StorageWidget,
    map_widget: MapWidget,
    player_widget: PlayerWidget,
    composition_widget: CompositionWidget,
    settings: Rc<RefCell<Settings>>,
    last_upd: std::time::Instant,
}

impl ApplicationImp {
    /// Create main window struct
    pub fn new(application: Application, settings: Rc<RefCell<Settings>>) -> ApplicationImp {
        let application = Rc::new(RefCell::new(application));
        let storage = application.borrow().get_storage();
        let map = application.borrow().get_root_map();
        let player = application.borrow().get_player();
        ApplicationImp {
            application: Rc::clone(&application),
            events: VecDeque::new(),
            storage_widget: StorageWidget::new(storage, Rc::clone(&application)),
            map_widget: MapWidget::new(map, Rc::clone(&application)),
            player_widget: PlayerWidget::new(player),
            composition_widget: CompositionWidget::new(Rc::clone(&application)),
            settings,
            last_upd: std::time::Instant::now(),
        }
    }

    fn handle_events(&mut self, ctx: &egui::Context) {
        while let Some(event) = self.events.pop_front() {
            match event {
                Event::SetupStorage { credentials } => {
                    // TODO: show error message
                    let setup_error = self.application.borrow_mut().setup_storage(credentials);
                    if let Ok(_) = setup_error {
                        let storage = self.application.borrow().get_storage();
                        let map = self.application.borrow().get_root_map();
                        let player = self.application.borrow().get_player();
                        self.storage_widget =
                            StorageWidget::new(storage, Rc::clone(&self.application));
                        self.map_widget = MapWidget::new(map, Rc::clone(&self.application));
                        self.player_widget = PlayerWidget::new(player);
                        self.composition_widget =
                            CompositionWidget::new(Rc::clone(&self.application));
                    }
                }
                Event::Play { audio } => {
                    self.player_widget.play(&audio);
                    self.application.borrow_mut().player_set_audio(audio);
                    self.application.borrow_mut().player_play();
                }
                Event::AddAudioToComposition { audio } => {
                    self.composition_widget.insert_audio(Rc::clone(&audio));
                    self.application.borrow_mut().player_sync();
                }
                Event::PlayerSync => {
                    self.application.borrow_mut().player_sync();
                    if let Some(playing) = self
                        .application
                        .borrow()
                        .get_current_playing()
                        .borrow()
                        .as_ref()
                    {
                        self.player_widget.play(playing);
                    }
                }
                Event::PlayerSetVolume { volume } => {
                    self.application.borrow_mut().player_set_volume(volume)
                }
                Event::PlayerSetTrackVolume {
                    volume,
                    composition_index,
                    index,
                } => self.application.borrow_mut().player_set_track_volume(
                    volume,
                    composition_index,
                    index,
                ),

                Event::Select { audio } => {
                    self.application
                        .borrow_mut()
                        .set_selected_composition(Some(audio));
                    self.composition_widget.sync_with_application();
                }
                Event::SaveProject { path } => {
                    match self.application.borrow_mut().save_project(path) {
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
                    }
                }
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

        // FPS controller
        let now = std::time::Instant::now();
        let fps = 60;
        let goal_delay = std::time::Duration::from_secs(1) / fps;
        let delta = now - self.last_upd;
        if delta < goal_delay {
            std::thread::sleep(goal_delay - delta);
        }
        self.last_upd = now;
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
