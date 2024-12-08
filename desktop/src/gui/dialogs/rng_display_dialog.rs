use crate::gui::text;
use chrono::DateTime;
use egui::{Align2, Grid, Image};
use unic_langid::LanguageIdentifier;

/// Renders the About Ruffle dialog.
pub fn show_rng_display_dialog(locale: &LanguageIdentifier, egui_ctx: &egui::Context) -> bool {
    let mut keep_open = true;

    egui::Window::new(text(locale, "rng-display-dialog"))
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .open(&mut keep_open)
        .show(egui_ctx, |ui| {
            ui.vertical_centered(|ui| {
                Grid::new("rng_display_dialog_grid")
                    .striped(true)
                    .show(ui, |ui| {


                        /* 
                            TODO: THIS SHOULD USE LOCALIZATION STUFF! I don't imagine this'll get merged so it won't be translated,
                            but I should still move the english text there.
                        */
                        ui.label(text(locale, "rng-display-frame-txt"));


                        ui.label(text(locale, "rng-display-frame-rng"));
                        ui.end_row();

                    });
            })
        });

    keep_open
}
