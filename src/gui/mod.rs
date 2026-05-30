//! GUI module using egui/eframe

mod app;
pub mod theme;
pub mod fonts;

pub use app::MarkItDownApp;

/// Run the GUI application
pub fn run_gui() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([900.0, 650.0]),
        ..Default::default()
    };

    eframe::run_native(
        "MarkItDown-RST",
        options,
        Box::new(|cc| {
            crate::gui::theme::Theme::apply(&cc.egui_ctx, true);
            crate::gui::fonts::install(&cc.egui_ctx);
            Ok(Box::new(MarkItDownApp::new()))
        }),
    )
}
