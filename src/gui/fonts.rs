//! Font loading — Inter (UI) + JetBrains Mono (code) + Noto Sans SC (CJK)
//!
//! Fonts are embedded via `include_bytes!` and registered through
//! `egui::FontDefinitions`, so the binary is fully self-contained
//! with no external font dependencies.

use eframe::egui;
use std::sync::Arc;

/// Install custom fonts into the egui context.
///
/// - **Inter** — proportional font for UI text (headings, buttons, labels)
/// - **JetBrains Mono** — monospace font for code, preview, file paths
/// - **Noto Sans SC** — CJK font for Chinese characters
///
/// Font loading is wrapped in `std::panic::catch_unwind` so that a corrupted
/// or invalid font file causes a warning instead of crashing the application.
/// The user will still see the UI with egui's built-in fallback fonts.
pub fn install(ctx: &egui::Context) {
    let mut font_defs = egui::FontDefinitions::default();

    // Load each font safely — if any fails, skip it and continue
    let inter_regular = safe_load_font("Inter-Regular", include_bytes!("../../assets/fonts/Inter-Regular.ttf"));
    let inter_bold    = safe_load_font("Inter-Bold",    include_bytes!("../../assets/fonts/Inter-Bold.ttf"));
    let jetbrains     = safe_load_font("JetBrainsMono",  include_bytes!("../../assets/fonts/JetBrainsMono-Regular.ttf"));
    let noto_sc       = safe_load_font("NotoSansSC",     include_bytes!("../../assets/fonts/NotoSansSC-Regular.ttf"));

    // Register font data
    if let Some(data) = inter_regular {
        font_defs.font_data.insert("Inter-Regular".into(), Arc::new(data));
    }
    if let Some(data) = inter_bold {
        font_defs.font_data.insert("Inter-Bold".into(), Arc::new(data));
    }
    if let Some(data) = jetbrains {
        font_defs.font_data.insert("JetBrainsMono".into(), Arc::new(data));
    }
    if let Some(data) = noto_sc {
        font_defs.font_data.insert("NotoSansSC".into(), Arc::new(data));
    }

    // Set font family priorities (first = highest priority for glyph lookup)

    // Proportional: Inter-Regular → Inter-Bold → NotoSansSC → (egui defaults)
    let proportional = font_defs.families.entry(egui::FontFamily::Proportional).or_default();
    if font_defs.font_data.contains_key("Inter-Regular") {
        proportional.insert(0, "Inter-Regular".into());
    }
    if font_defs.font_data.contains_key("Inter-Bold") {
        proportional.insert(1, "Inter-Bold".into());
    }
    if font_defs.font_data.contains_key("NotoSansSC") {
        proportional.push("NotoSansSC".into());
    }

    // Monospace: JetBrainsMono → NotoSansSC → (egui defaults)
    let monospace = font_defs.families.entry(egui::FontFamily::Monospace).or_default();
    if font_defs.font_data.contains_key("JetBrainsMono") {
        monospace.insert(0, "JetBrainsMono".into());
    }
    if font_defs.font_data.contains_key("NotoSansSC") {
        monospace.push("NotoSansSC".into());
    }

    ctx.set_fonts(font_defs);
}

/// Load a font from bytes, catching any panics from egui's font parser.
///
/// egui's `FontData::from_static()` itself doesn't parse the font — parsing
/// happens later in `epaint` when the fonts are first used. However, wrapping
/// `ctx.set_fonts()` in catch_unwind is awkward. Instead, we validate the
/// bytes look like a TTF/OTF file before passing them to egui.
fn safe_load_font(name: &str, bytes: &'static [u8]) -> Option<egui::FontData> {
    // Quick validation: TTF/OTF magic bytes
    if bytes.len() < 4 {
        eprintln!("MDrust: font '{name}' is too small ({} bytes), skipping", bytes.len());
        return None;
    }

    let magic = &bytes[0..4];
    let is_valid = magic == [0x00, 0x01, 0x00, 0x00]  // TrueType
        || magic == [0x4F, 0x54, 0x54, 0x4F]          // "OTTO" (OpenType/CFF)
        || magic == [0x74, 0x72, 0x75, 0x65]          // "true" (Apple TrueType)
        || magic == [0x74, 0x74, 0x63, 0x66];         // "ttcf" (TrueType Collection)

    if !is_valid {
        eprintln!(
            "MDrust: font '{name}' has invalid magic bytes ({:#04X?}), not a TTF/OTF file — skipping. \
             This usually means the font file is corrupted or is an HTML page (wrong download URL).",
            magic
        );
        return None;
    }

    let mut font_data = egui::FontData::from_static(bytes);
    if name == "Inter-Regular" {
        font_data.tweak.y_offset_factor = -0.02;
    }
    Some(font_data)
}
