use eframe::egui;

// The core editor for getting highlighted notes
#[derive(Clone)]
pub struct NoteTextEditor {
    pub note: String,
}

impl Default for NoteTextEditor {
    fn default() -> Self {
        Self {
            note: "".into(),
        }
    }
}

impl egui::Widget for &mut NoteTextEditor {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = crate::highlighter::CodeTheme::from_memory(ui.ctx());

        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            let mut layout_job =
                crate::highlighter::highlight(ui.ctx(), &theme, string, &"note".to_owned());
            layout_job.wrap.max_width = wrap_width;
            ui.fonts().layout_job(layout_job)
        };

        let resp = ui.add(
            egui::TextEdit::multiline(&mut self.note)
                .font(egui::TextStyle::Monospace)
                .desired_rows(10)
                .lock_focus(true)
                .desired_width(f32::INFINITY)
                .layouter(&mut layouter),
        );

        resp
    }
}

