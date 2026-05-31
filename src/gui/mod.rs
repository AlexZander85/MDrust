//! GUI module using egui/eframe

mod app;
pub mod theme;
pub mod fonts;

pub use app::MarkItDownApp;

use std::sync::Arc;

/// Load the application icon from embedded PNG bytes
fn load_icon() -> Option<egui::IconData> {
    let png_bytes = include_bytes!("../../assets/icon-256.png");
    let img = match image::load_from_memory(png_bytes) {
        Ok(img) => img,
        Err(e) => {
            tracing::warn!("Failed to load window icon: {e}");
            return None;
        }
    };
    let img = img.resize(256, 256, image::imageops::FilterType::Lanczos3);
    let rgba = img.to_rgba8();
    let width = rgba.width();
    let height = rgba.height();
    Some(egui::IconData {
        rgba: rgba.into_raw(),
        width,
        height,
    })
}

/// Run the GUI application with automatic renderer fallback:
///
/// 1. Try **wgpu** (default — DirectX 11/12 on Windows, Vulkan/Metal on Linux/macOS)
/// 2. If wgpu fails, fall back to **glow** (OpenGL 3.0+ — works on almost any GPU)
///
/// This ensures the app works even on systems with outdated GPU drivers
/// or incompatible graphics hardware.
pub fn run_gui() -> eframe::Result<()> {
    let icon = load_icon();

    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([1200.0, 800.0])
        .with_min_inner_size([900.0, 650.0]);

    if let Some(icon_data) = icon {
        viewport = viewport.with_icon(Arc::new(icon_data));
    }

    // First attempt: wgpu renderer (default, best performance)
    let options = eframe::NativeOptions {
        viewport,
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    let result = eframe::run_native(
        "MDrust",
        options,
        Box::new(|cc| {
            crate::gui::theme::Theme::apply(&cc.egui_ctx, true);
            crate::gui::fonts::install(&cc.egui_ctx);
            Ok(Box::new(MarkItDownApp::new()))
        }),
    );

    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            let err_str = format!("{e}");

            // If wgpu failed, try glow (OpenGL) as a fallback
            if err_str.contains("wgpu") || err_str.contains("gpu") || err_str.contains("adapter") || err_str.contains("device") || err_str.contains("graphics") {
                eprintln!("MDrust: wgpu renderer failed ({err_str}), trying glow (OpenGL) fallback...");

                let icon = load_icon();
                let mut viewport = egui::ViewportBuilder::default()
                    .with_inner_size([1200.0, 800.0])
                    .with_min_inner_size([900.0, 650.0]);

                if let Some(icon_data) = icon {
                    viewport = viewport.with_icon(Arc::new(icon_data));
                }

                let fallback_options = eframe::NativeOptions {
                    viewport,
                    renderer: eframe::Renderer::Glow,
                    ..Default::default()
                };

                return eframe::run_native(
                    "MDrust (OpenGL)",
                    fallback_options,
                    Box::new(|cc| {
                        crate::gui::theme::Theme::apply(&cc.egui_ctx, true);
                        crate::gui::fonts::install(&cc.egui_ctx);
                        Ok(Box::new(MarkItDownApp::new()))
                    }),
                );
            }

            // Non-GPU error — propagate it (main.rs will show MessageBox on Windows)
            Err(e)
        }
    }
}
