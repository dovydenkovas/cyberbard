use egui::Ui;

pub struct SettingsWidget {
    title: String,
}

impl SettingsWidget {
    pub fn new() -> SettingsWidget {
        SettingsWidget {
            title: "Основная тема".to_string(),
        }
    }

    pub fn update(&mut self, _ctx: &egui::Context, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading(&self.title);
        });

        ui.horizontal(|ui| {
            let mut scalar = 0.0;
            ui.label("Громкость");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add(egui::Slider::new(&mut scalar, 0.0..=100.0).show_value(false));
            });
        });

        ui.horizontal(|ui| {
            let mut scalar = 0.0;
            ui.label("Цикличность");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.button("🔁");
            });
        });

        ui.add_space(20.0);
        ui.vertical_centered(|ui| {
            ui.label("Состав композиции");
            ui.add_space(10.0);
        });

        ui.horizontal(|ui| {
            let mut scalar = 0.0;
            ui.label("The Shire");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.small("🗙");
                ui.button("🔁");
                ui.add(egui::Slider::new(&mut scalar, 0.0..=100.0).show_value(false));
            });
        });
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            let mut scalar = 0.0;
            ui.label("Rain");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.small("🗙");
                ui.button("🔁");
                ui.add(egui::Slider::new(&mut scalar, 0.0..=100.0).show_value(false));
            });
        });
    }
}
