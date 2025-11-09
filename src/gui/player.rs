use egui::Ui;

pub struct PlayerWidget {
    title: String,
    progress: f32,
    volume: f32,
    is_pause: bool,
    is_looped: bool,
}

impl PlayerWidget {
    pub fn new() -> PlayerWidget {
        PlayerWidget {
            title: "The Shire".to_string(),
            progress: 0.6,
            volume: 100.0,
            is_pause: false,
            is_looped: false,
        }
    }

    fn toggle_pause(&mut self) {
        self.is_pause = !self.is_pause;
    }

    fn reset(&mut self) {}

    fn toggle_looped(&mut self) {
        self.is_looped = !self.is_looped;
    }

    pub fn update(&mut self, _ctx: &egui::Context, ui: &mut Ui) {
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
                self.reset();
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
