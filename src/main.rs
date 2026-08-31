mod info;
use eframe::egui;
use egui::{RichText};
use crate::info::VideoInfo;

#[derive(Default)]
struct MyApp {
    info: Option<VideoInfo>
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.set_theme(egui::ThemePreference::System);
            ui.label(RichText::new("Welcome to open media player!").size(25.0));
            ui.separator();
            ui.heading("MP4 info:");

            match &self.info {
                Some(v) => {
                    ui.label(format!("Resolution: {} x {}", v.resolution.0, v.resolution.1));
                    ui.label(format!("Framerate: {:.2} fps", f64::from(v.framerate)));
                    ui.label(format!("Duration: {} sec ({} min)", v.durationSec, v.durationMin));
                }
                _ => {
                    ui.label("Couldn't fetch mp4 info!");
                }
            }
        });
    }
}

fn main() -> eframe::Result {
    let info = info::get_info("test.mp4");

    eframe::run_native(
        "Open Media Player",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp { info }))),
    )
}