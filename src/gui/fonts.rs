//! Font loading

/// Install custom fonts.
pub fn install(ctx: &egui::Context) {
    // Use default egui fonts for now — they include good Unicode coverage.
    // Phosphor icons are not available due to version compatibility,
    // so we use Unicode symbols for icons instead.
    let _ = ctx;
}
