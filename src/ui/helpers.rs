use egui::{Color32, Response, RichText, Rounding, Ui};

pub fn help_marker(ui: &mut Ui, hover_text: &str) -> Response {
    ui.colored_label(Color32::from_rgb(130, 160, 210), "(?)")
        .on_hover_text(hover_text)
}

pub fn section_header(ui: &mut Ui, title: &str, subtitle: Option<&str>) {
    ui.label(RichText::new(title).size(17.0).strong().color(Color32::from_rgb(240, 243, 250)));
    if let Some(sub) = subtitle {
        ui.label(RichText::new(sub).size(12.5).color(Color32::from_rgb(160, 170, 190)));
    }
    ui.add_space(6.0);
    ui.separator();
    ui.add_space(8.0);
}

pub fn card_frame(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    egui::Frame::group(ui.style())
        .fill(Color32::from_rgb(26, 29, 36))
        .stroke(egui::Stroke::new(1.0, Color32::from_rgb(45, 50, 62)))
        .inner_margin(egui::Margin::same(12.0))
        .rounding(Rounding::same(6.0))
        .show(ui, add_contents);
}
