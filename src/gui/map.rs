use egui::{Ui, Vec2, load::SizedTexture, vec2};

#[derive(Clone)]
struct Child {
    pub title: String,
    pub x: f32,
    pub y: f32,
}

pub struct MapWidget {
    image: Option<egui::ColorImage>,
    compositions: Vec<String>,
    childs: Vec<Child>,
}

impl MapWidget {
    pub fn new() -> MapWidget {
        MapWidget {
            image: None,
            compositions: vec![
                "Основная тема".to_string(),
                "Битва".to_string(),
                "Трагическая тема".to_string(),
                "Диалог".to_string(),
            ],
            childs: vec![
                Child {
                    title: "1".to_string(),
                    x: 0.3,
                    y: 0.15,
                },
                Child {
                    title: "2".to_string(),
                    x: 0.8,
                    y: 0.3,
                },
                Child {
                    title: "3".to_string(),
                    x: 0.5,
                    y: 0.5,
                },
                Child {
                    title: "4".to_string(),
                    x: 0.6,
                    y: 0.9,
                },
            ],
        }
    }

    fn goto_parent_map(&mut self) {
        println!("goto parent");
    }

    fn goto_child_map(&mut self, title: String) {
        println!("goto child {}", title)
    }

    fn select_composition(&self, text: &str) {
        println!("Select {}", text)
    }

    fn add_composition(&mut self) {
        println!("Add composition");
    }

    fn update_composition(&self, ui: &mut Ui, text: &str) {
        let btn = egui::Button::new(text).min_size(Vec2::new(80.0, 50.0));
        if ui.add(btn).clicked() {
            self.select_composition(text);
        }
        ui.add_space(5.0);
    }

    pub fn update(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        egui::SidePanel::left("Tracks")
            .resizable(false)
            .default_width(150.0)
            .show_inside(ui, |ui| {
                if ui.button("⏴").clicked() {
                    self.goto_parent_map();
                }
                ui.vertical_centered_justified(|ui| {
                    ui.add_space(20.0);
                    for composition in &self.compositions {
                        self.update_composition(ui, composition);
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
            });
        ui.add_space(10.0);

        if self.image.is_none() {
            let image_data = load_image_from_path("music/bg.png").unwrap();
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
                        egui::Button::new(&child.title)
                            .corner_radius(90.0)
                            .min_size(vec2(0.0, 0.0))
                            .fill(egui::Color32::from_rgb(0, 0, 80)),
                    );

                    if response.clicked() {
                        self.goto_child_map(child.title)
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
