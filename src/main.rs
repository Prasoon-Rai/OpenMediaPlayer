mod info;
use eframe::egui;
use egui::{RichText};
use crate::info::{get_info, VideoInfo};
use rfd::FileDialog;

#[derive(Default)]
struct UI {
    video_path: Option<String>,
    info: Option<VideoInfo>
}

impl eframe::App for UI {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.set_theme(egui::ThemePreference::System);
            ui.label(RichText::new("Welcome to open media player!").size(25.0));
            ui.separator();

            if ui.button("Open Video File").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("Video Files", &["mp4", "mkv", "avi", "mov"])
                    .pick_file()
                {
                    if let Some(path_str) = path.to_str() {
                        self.video_path = Some(path_str.to_string());
                        self.info = get_info(path_str);
                    }
                }
            }
            if let Some(path) = &self.video_path {
                ui.label(format!("Selcted file: {}", path));

                ui.separator();

                ui.heading("MP4 info:");

                match &self.info {
                    Some(v) => {
                        ui.label(format!("Resolution: {} x {}", v.resolution.0, v.resolution.1));
                        ui.label(format!("Framerate: {:.2} fps", f64::from(v.framerate)));

                        ui.label(format!("Duration: {} sec", v.durationSec));
                    }
                    None => {
                        ui.label("Couldn't fetch video info!");
                    }
                }
            }

        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "Open Media Player",
        eframe::NativeOptions::default(),
        Box::new(move |_cc| Ok(Box::new(UI::default()))),
    )
}