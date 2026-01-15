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

use std::{cell::RefCell, rc::Rc};

use crate::{
    audio::audio::Audio,
    gui::events::{Event, Events},
    map::{Map, Point},
};
use egui::{Label, Sense, Ui, Vec2, load::SizedTexture, vec2};
use rfd::FileDialog;

pub struct MapWidget {
    map: Rc<RefCell<Map>>,
    is_root: bool,
    hide_map: bool,
    show_open_map_dialog: bool,
}

impl MapWidget {
    pub fn new(map: Rc<RefCell<Map>>) -> MapWidget {
        MapWidget {
            map,
            is_root: true,
            hide_map: false,
            show_open_map_dialog: false,
        }
    }

    fn goto_parent_map(&mut self) {
        let map = self.map.borrow().get_parent();
        if map.is_some() {
            self.map = map.unwrap();
            self.is_root = self.map.borrow().get_parent().is_none();
            self.hide_map = self.map.borrow().get_background().is_none();
        }
    }

    fn goto_child_map(&mut self, point: Point) {
        let map = self.map.borrow().get_map(&point);
        if map.is_some() {
            self.map = map.unwrap();
            self.is_root = self.map.borrow().get_parent().is_none();
            self.hide_map = self.map.borrow().get_background().is_none();
        }
    }

    fn remove_child_map(&mut self, point: Point) {
        self.map.borrow_mut().erase_map(point);
    }

    fn select_composition(&self, audio: Audio, events: &mut Events) {
        events.push_back(Event::Play {
            audio: Rc::clone(&audio),
        });
        events.push_back(Event::Select { audio });
    }

    fn add_composition(&mut self) {
        self.map.borrow_mut().push_new_audio();
    }

    pub fn update(&mut self, ctx: &egui::Context, ui: &mut Ui, events: &mut Events) {
        if self.hide_map {
            self.render_playlists(ctx, ui, events);
        } else {
            egui::SidePanel::left("Tracks")
                .resizable(true)
                .default_width(200.0)
                .show_inside(ui, |ui| {
                    self.render_playlists(ctx, ui, events);
            });
            ui.add_space(10.0);
            self.render_map_widget(ctx, ui);
        }
    }

    fn render_playlists(&mut self, _ctx: &egui::Context, ui: &mut Ui, events: &mut Events) {
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let hide_label = "↔";
                if ui.button(hide_label.to_string()).clicked() {
                    self.hide_map = !self.hide_map;
                };
                if !self.is_root {
                    if ui.button("⬆").clicked() {
                        self.goto_parent_map();
                    }
                }
                ui.vertical(|ui| {
                    // TODO: not real centered for now
                    ui.centered_and_justified(|ui| {
                        ui.heading("Плейлисты");
                    });
                });
            });
        });

        ui.vertical_centered_justified(|ui| {
            ui.add_space(20.0);
            let mut remove_after_render = None;
            for i in 0..self.map.borrow().audio_count() {
                self.render_composition(ui, &self.map, i, events, &mut remove_after_render);
            }

            if let Some(index) = remove_after_render {
                self.map.borrow_mut().erase_audio(index);
            }
        });

        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            let btn = egui::Button::new("+")
                .min_size(Vec2::new(40.0, 40.0))
                .corner_radius(90.0);
            if ui.add(btn).clicked() {
                self.add_composition();
            }
        });
    }

    fn render_composition(
        &self,
        ui: &mut Ui,
        map: &Rc<RefCell<Map>>,
        index: usize,
        events: &mut Events,
        remove_after_render: &mut Option<usize>,
    ) {
        let audio = map.borrow().get_audio(index);
        let title = audio.borrow().get_title();

        let btn = egui::Button::new(&title).min_size(Vec2::new(80.0, 50.0));

        let response = ui.add(btn);
        if response.clicked() {
            self.select_composition(audio, events);
        }
        if response.secondary_clicked() {
            remove_after_render.replace(index);
        }
        ui.add_space(5.0);
    }

    fn render_map_widget(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        if self.show_open_map_dialog {
            self.render_open_map_dialog(ctx, ui);
        } else if self.map.borrow().get_background().is_none() {
            ui.centered_and_justified(|ui| {
                let btn = Label::new("Добавить карту".to_string()).sense(Sense::click());
                if ui.add(btn).clicked() {
                    self.show_open_map_dialog = true;
                }
            });
        } else {
            ui.centered_and_justified(|ui| {
                self.render_map(ctx, ui);
            });
        }
    }

    fn render_map(&mut self, _ctx: &egui::Context, ui: &mut Ui) {
        assert!(self.map.borrow().get_background().is_some());
        let child_radius = 0.05;

        let image = &self.map.borrow().get_background().unwrap();

        let available_size = ui.available_size();
        let (w, h) = (image.size()[0] as f32, image.size()[1] as f32);

        // Hide if too small
        if available_size.x < 50.0 || available_size.y < 50.0 {
            return;
        }

        // Scale to avaliable size
        let (w, h) = scale_texture(available_size.x, available_size.y, w, h);
        let scaled_texture = SizedTexture::new(image.id(), vec2(w, h));
        let o = ui.next_widget_position();
        let image_widget = egui::Image::from_texture(scaled_texture).sense(Sense::click());
        let response = ui.add(image_widget);

        // Track mouse
        if response.secondary_clicked()
            && let Some(pos) = response.interact_pointer_pos()
        {
            let x = (pos.x - o.x - 0.5 * child_radius * w) / w + 0.5;
            let y = (pos.y - o.y - 0.5 * child_radius * w) / h + 0.5;
            let parent = Rc::new(RefCell::new(Map::new(Some(Rc::clone(&self.map)))));
            self.map.borrow_mut().insert_map(Point { x, y }, parent);
        }

        // Render childs
        // TODO: control childs color (logo?) and size
        let mut clicked = None;
        let mut removed = None;
        for child in self.map.borrow().iter_maps() {
            let button_rect = egui::Rect::from_min_size(
                egui::pos2(o.x + (child.x - 0.5) * w, o.y + (child.y - 0.5) * h),
                vec2(child_radius * w, child_radius * w),
            );

            let response = ui.put(
                button_rect,
                egui::Button::new("")
                    .corner_radius(90.0)
                    .min_size(vec2(0.0, 0.0))
                    .fill(egui::Color32::from_hex("#b67404aa").unwrap()),
            );

            if response.clicked() {
                clicked = Some(child.clone());
            }
            if response.secondary_clicked() {
                removed = Some(child.clone());
            }
        }
        if let Some(child) = clicked {
            self.goto_child_map(child);
        }
        if let Some(child) = removed {
            self.remove_child_map(child);
        }
    }

    fn render_open_map_dialog(&mut self, ctx: &egui::Context, _ui: &mut Ui) {
        // TODO: not to lock the main window
        let path = FileDialog::new()
            .set_title("Выбор каталога с музыкой и файлами игры")
            .add_filter("Image", &["png", "jpg", "jpeg", "webp", "bmp"])
            .pick_file();

        if let Some(path) = path {
            // TODO: add load animation
            match load_image_from_path(path.to_str().unwrap()) {
                // TODO: cut map rectangle from image
                Ok(image_data) => {
                    let handle = ctx.load_texture(
                        path.to_str().unwrap(), // A unique name for the texture
                        image_data,
                        Default::default(),
                    );
                    self.map.borrow_mut().set_background(Some(handle));
                }
                Err(_) => (), // TODO: add error message
            }
        }
        self.show_open_map_dialog = false;
    }
}

/// Return new (width, height) to draw image inside window.
fn scale_texture(win_w: f32, win_h: f32, w: f32, h: f32) -> (f32, f32) {
    if w / h > win_w / win_h {
        (win_w, h * win_w / w)
    } else {
        (w * win_h / h, win_h)
    }
}


/// Helper function to load image data
fn load_image_from_path(path: &str) -> Result<egui::ColorImage, String> {
    // TODO: fix performance.
    let image = image::open(path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .to_rgba8();
    let (width, height) = image.dimensions();

    let texture = egui::ColorImage::from_rgba_premultiplied(
        [width as usize, height as usize],
        &image.to_vec(),
    );

    Ok(texture)
}
