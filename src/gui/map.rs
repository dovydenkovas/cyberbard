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
use egui::{Color32, ColorImage, Ui, Vec2, load::SizedTexture, vec2};

pub struct MapWidget {
    map: Rc<RefCell<Map>>,
    image: Option<egui::ColorImage>,
    childs: Vec<Point>,
    is_root: bool,
}

impl MapWidget {
    pub fn new(map: Rc<RefCell<Map>>) -> MapWidget {
        MapWidget {
            map,
            image: None,
            childs: vec![],
            is_root: true,
        }
    }

    fn goto_parent_map(&mut self) {
        println!("goto parent");
    }

    fn goto_child_map(&mut self, point: Point) {
        println!("goto child {:?}", point)
    }

    fn select_composition(&self, audio: Rc<RefCell<dyn Audio>>, events: &mut Events) {
        events.push_back(Event::Play {
            audio: Rc::clone(&audio),
        });
        events.push_back(Event::Select { audio });
    }

    fn add_composition(&mut self, events: &mut Events) {
        events.push_back(Event::MapNewComposition);
    }

    fn render_composition(
        &self,
        ui: &mut Ui,
        map: &Rc<RefCell<Map>>,
        index: usize,
        events: &mut Events,
    ) {
        let audio = map.borrow().get_audio(index);
        let title = audio.borrow().get_title();

        let btn = egui::Button::new(&title).min_size(Vec2::new(80.0, 50.0));

        let response = ui.add(btn);
        if response.clicked() {
            self.select_composition(audio, events);
        }
        ui.add_space(5.0);
    }

    pub fn update(&mut self, ctx: &egui::Context, ui: &mut Ui, events: &mut Events) {
        if self.image.is_none() {
            self.render_tracks(ctx, ui, events);
        } else {
            egui::SidePanel::left("Tracks")
                .resizable(false)
                .default_width(150.0)
                .show_inside(ui, |ui| {
                    self.render_tracks(ctx, ui, events);
                });
            ui.add_space(10.0);
            self.render_map(ctx, ui);
        }
    }

    fn render_tracks(&mut self, _ctx: &egui::Context, ui: &mut Ui, events: &mut Events) {
        if !self.is_root {
            if ui.button("⏴").clicked() {
                self.goto_parent_map();
            }
        }

        ui.vertical_centered_justified(|ui| {
            ui.add_space(20.0);
            for i in 0..self.map.borrow().audio_count() {
                self.render_composition(ui, &self.map, i, events);
            }
        });

        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            let btn = egui::Button::new("+")
                .min_size(Vec2::new(40.0, 40.0))
                .corner_radius(90.0);
            if ui.add(btn).clicked() {
                self.add_composition(events);
            }
        });
    }

    fn render_map(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        if self.image.is_none() {
            let image_data = load_image_from_path("music/bg.png")
                .unwrap_or(ColorImage::filled([300, 300], Color32::from_white_alpha(0)));
            self.image = Some(image_data);
        }

        ui.centered_and_justified(|ui| {
            if let Some(texture) = &self.image {
                let handle = ctx.load_texture(
                    "my-image-name", // A unique name for the texture
                    texture.clone(),
                    Default::default(), // Texture options
                );

                let available_size = ui.available_size();
                let (mut w, mut h) = (handle.size()[0] as f32, handle.size()[1] as f32);
                if available_size.x < 50.0 || available_size.y < 50.0 {
                    return;
                }
                if available_size.x / available_size.y > h / w {
                    let k = handle.size()[1] as f32 / handle.size()[0] as f32;
                    w = k * available_size.y;
                    h = available_size.y;
                } else {
                    let k = handle.size()[0] as f32 / handle.size()[1] as f32;
                    w = available_size.x;
                    h = k * available_size.x;
                };
                let sized_texture = SizedTexture::new(handle.id(), vec2(w, h));
                let pos = ui.next_widget_position();
                ui.add(egui::Image::from_texture(sized_texture));

                for child in self.childs.clone() {
                    let button_rect = egui::Rect::from_min_size(
                        egui::pos2(pos.x + (child.x - 0.5) * w, pos.y + (child.y - 0.5) * h),
                        vec2(0.1 * w, 0.1 * w),
                    );
                    let response = ui.put(
                        button_rect,
                        egui::Button::new("")
                            .corner_radius(90.0)
                            .min_size(vec2(0.0, 0.0))
                            .fill(egui::Color32::from_rgb(0, 0, 80)),
                    );

                    if response.clicked() {
                        self.goto_child_map(child)
                    }
                }
            } else {
                ui.label("Загрузка карты...");
            }
        });
    }
}

// Helper function to load image data (replace with your preferred image loading library)
fn load_image_from_path(path: &str) -> Result<egui::ColorImage, String> {
    // Example using the `image` crate (requires adding `image = "0.24"` to Cargo.toml)
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
