#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

pub fn run() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "rf2 Mod Creator LogWindow",
        options,
        Box::new(|_cc| Box::<LogWindowApp>::default()),
    ).unwrap()
}

struct LogWindowApp {
    log: String,
}

impl Default for LogWindowApp {
    fn default() -> Self {
        Self {
            log: "".to_owned(),
        }
    }
}

impl eframe::App for LogWindowApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //
            ui.heading("My egui Application");
            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name)
            //         .labelled_by(name_label.id);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Click each year").clicked() {
            //     self.age += 1;
            // }
            ui.label(self.log.clone());
        });
    }
}