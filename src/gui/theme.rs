//! Design system with colors for dark/light themes

use egui::{Color32, FontId, FontFamily, Rounding, Stroke, TextStyle, Visuals};

pub struct Theme;

impl Theme {
    // ── Dark palette ──────────────────────────────────────────────────────
    pub const BG:        Color32 = Color32::from_rgb(18, 20, 24);
    pub const SURFACE:   Color32 = Color32::from_rgb(28, 31, 38);
    pub const SURFACE_2: Color32 = Color32::from_rgb(36, 40, 48);
    pub const BORDER:    Color32 = Color32::from_rgb(50, 55, 65);
    pub const TEXT:      Color32 = Color32::from_rgb(228, 230, 235);
    pub const TEXT_DIM:  Color32 = Color32::from_rgb(140, 148, 160);
    pub const ACCENT:    Color32 = Color32::from_rgb(99, 134, 255);
    pub const ACCENT_HI: Color32 = Color32::from_rgb(130, 159, 255);
    pub const SUCCESS:   Color32 = Color32::from_rgb(80, 200, 130);
    pub const WARNING:   Color32 = Color32::from_rgb(255, 184, 80);
    pub const ERROR:     Color32 = Color32::from_rgb(255, 95, 95);

    // ── Light palette ─────────────────────────────────────────────────────
    pub const LIGHT_BG:        Color32 = Color32::from_rgb(245, 245, 248);
    pub const LIGHT_SURFACE:   Color32 = Color32::from_rgb(255, 255, 255);
    pub const LIGHT_SURFACE_2: Color32 = Color32::from_rgb(235, 237, 242);
    pub const LIGHT_BORDER:    Color32 = Color32::from_rgb(210, 213, 220);
    pub const LIGHT_TEXT:      Color32 = Color32::from_rgb(30, 32, 38);
    pub const LIGHT_TEXT_DIM:  Color32 = Color32::from_rgb(100, 106, 118);
    pub const LIGHT_ACCENT:    Color32 = Color32::from_rgb(60, 100, 220);

    pub fn apply(ctx: &egui::Context, dark: bool) {
        let mut visuals = if dark { Visuals::dark() } else { Visuals::light() };
        if dark {
            visuals.panel_fill = Self::BG;
            visuals.window_fill = Self::SURFACE;
            visuals.extreme_bg_color = Self::BG;
            visuals.faint_bg_color = Self::SURFACE_2;
            visuals.widgets.noninteractive.bg_fill = Self::SURFACE;
            visuals.widgets.inactive.bg_fill = Self::SURFACE_2;
            visuals.widgets.hovered.bg_fill = Self::SURFACE_2;
            visuals.widgets.active.bg_fill = Self::ACCENT;
            visuals.selection.bg_fill = Self::ACCENT.gamma_multiply(0.4);
            visuals.hyperlink_color = Self::ACCENT_HI;
            visuals.override_text_color = Some(Self::TEXT);
            visuals.window_stroke = Stroke::new(1.0, Self::BORDER);
        } else {
            visuals.panel_fill = Self::LIGHT_BG;
            visuals.window_fill = Self::LIGHT_SURFACE;
            visuals.extreme_bg_color = Self::LIGHT_BG;
            visuals.faint_bg_color = Self::LIGHT_SURFACE_2;
            visuals.widgets.noninteractive.bg_fill = Self::LIGHT_SURFACE;
            visuals.widgets.inactive.bg_fill = Self::LIGHT_SURFACE_2;
            visuals.widgets.hovered.bg_fill = Self::LIGHT_SURFACE_2;
            visuals.widgets.active.bg_fill = Self::LIGHT_ACCENT;
            visuals.selection.bg_fill = Self::LIGHT_ACCENT.gamma_multiply(0.3);
            visuals.hyperlink_color = Self::LIGHT_ACCENT;
            visuals.override_text_color = Some(Self::LIGHT_TEXT);
            visuals.window_stroke = Stroke::new(1.0, Self::LIGHT_BORDER);
        }
        ctx.set_visuals(visuals);

        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.button_padding = egui::vec2(12.0, 6.0);
        style.spacing.interact_size.y = 28.0;

        style.text_styles = [
            (TextStyle::Heading,   FontId::new(20.0, FontFamily::Proportional)),
            (TextStyle::Body,      FontId::new(13.5, FontFamily::Proportional)),
            (TextStyle::Button,    FontId::new(13.5, FontFamily::Proportional)),
            (TextStyle::Small,     FontId::new(11.5, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(12.5, FontFamily::Monospace)),
        ].into();

        ctx.set_style(style);
    }
}
