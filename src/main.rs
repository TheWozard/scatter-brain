#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, Widget};
use egui::{ModifierNames, Modifiers};

mod code;
mod highlighter;
mod token;

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Notes",
        options,
        Box::new(|_cc| Box::new(Core::default())),
    )
}

struct Core {
    editors: Vec<code::NoteTextEditor>
}

impl Default for Core {
    fn default() -> Self {
        Self {
            editors: vec![code::NoteTextEditor::default()],
        }
    }
}

impl eframe::App for Core {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            if ctx.input_mut().consume_key(Modifiers::NONE, egui::Key::Tab) {
                self.editors[0].note += "foo"
            }
            for editor in self.editors.iter_mut() {
                editor.ui(ui);
            }
        });
    }
}
