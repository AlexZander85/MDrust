//! Font loading — Inter (UI) + JetBrains Mono (code) + Noto Sans SC (CJK)
//!
//! Fonts are embedded via `include_bytes!` and registered through
//! `egui::FontDefinitions`, so the binary is fully self-contained
//! with no external font dependencies.

use eframe::egui;

/// Install custom fonts into the egui context.
///
/// - **Inter** — proportional font for UI text (headings, buttons, labels)
/// - **JetBrains Mono** — monospace font for code, preview, file paths
/// - **Noto Sans SC** — CJK font for Chinese characters
pub fn install(ctx: &egui::Context) {
    let mut fonts = egui::FontData::from_static(include_bytes!("../../assets/fonts/Inter-Regular.ttf"));
    fonts.tweak.y_offset_factor = -0.02; // slight baseline adjustment for Inter

    let inter_regular = egui::FontData::from_static(include_bytes!("../../assets/fonts/Inter-Bold.ttf"));
    let jetbrains_mono = egui::FontData::from_static(include_bytes!("../../assets/fonts/JetBrainsMono-Regular.ttf"));
    let noto_sans_sc = egui::FontData::from_static(include_bytes!("../../assets/fonts/NotoSansSC-Regular.ttf"));

    let mut font_defs = egui::FontDefinitions::default();

    // Register font data
    font_defs.font_data.insert(
        "Inter-Regular".into(),
        Arc::new(fonts),
    );
    font_defs.font_data.insert(
        "Inter-Bold".into(),
        Arc::new(inter_regular),
    );
    font_defs.font_data.insert(
        "JetBrainsMono".into(),
        Arc::new(jetbrains_mono),
    );
    font_defs.font_data.insert(
        "NotoSansSC".into(),
        Arc::new(noto_sans_sc),
    );

    // Set Inter as the primary proportional font (first = highest priority)
    font_defs.families.entry(egui::FontFamily::Proportional).or_default()
        .insert(0, "Inter-Regular".into());
    font_defs.families.entry(egui::FontFamily::Proportional).or_default()
        .insert(1, "Inter-Bold".into());
    // CJK fallback — Noto Sans SC covers Chinese, also has good coverage for Japanese/Korean
    font_defs.families.entry(egui::FontFamily::Proportional).or_default()
        .push("NotoSansSC".into());

    // Set JetBrains Mono as the primary monospace font
    font_defs.families.entry(egui::FontFamily::Monospace).or_default()
        .insert(0, "JetBrainsMono".into());
    // CJK fallback for monospace too
    font_defs.families.entry(egui::FontFamily::Monospace).or_default()
        .push("NotoSansSC".into());

    ctx.set_fonts(font_defs);
}

use std::sync::Arc;
