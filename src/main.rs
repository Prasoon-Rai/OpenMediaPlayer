mod info;
use eframe::egui;
use egui::{RichText, FontId, Color32};

#[derive(Default)]
struct MyApp {

}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.set_theme(egui::ThemePreference::System);
            ui.label(RichText::new("Welcome to open media player!").size(25.0));
            ui.separator();
            ui.heading("MP4 info:")
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "Open Media Player",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}