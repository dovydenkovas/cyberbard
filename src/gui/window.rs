use super::map::MapWidget;
use super::player::PlayerWidget;
use super::settings::SettingsWidget;
use super::storage::StoreWidget;

/// Describe Cyberbard main window.
/// Create and update all widgets and connect to core application.
pub struct Application {
    storage_widget: StoreWidget,
    map_widget: MapWidget,
    player_widget: PlayerWidget,
    settings_widget: SettingsWidget,
}

impl Application {
    /// Create main window struct
    pub fn new() -> Application {
        Application {
            storage_widget: StoreWidget::new(),
            map_widget: MapWidget::new(),
            player_widget: PlayerWidget::new(),
            settings_widget: SettingsWidget::new(),
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("Storage")
            .resizable(true)
            .default_width(300.0)
            .width_range(200.0..=400.0)
            .show(ctx, |ui| self.storage_widget.update(ctx, ui));

        egui::SidePanel::right("PlayerAndSettings")
            .resizable(true)
            .default_width(300.0)
            .min_width(160.0)
            .show(ctx, |ui| {
                self.player_widget.update(ctx, ui);
                ui.separator();
                self.settings_widget.update(ctx, ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.map_widget.update(ctx, ui);
        });
    }
}

pub fn run_gui() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Cyberbard",
        options,
        Box::new(|_cc| Ok(Box::new(Application::new()))),
    );
}
