use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// MarkItDown-RST — Multi-threaded Document-to-Markdown Converter
fn main() -> eframe::Result<()> {
    #[cfg(feature = "logs")]
    {
        use tracing_subscriber::EnvFilter;
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    }

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && (args[1] == "--cli" || args.contains(&"--cli".to_string())) {
        println!("Use markitdown-cli binary for command-line mode.");
        std::process::exit(0);
    }

    #[cfg(feature = "gui")]
    {
        markitdown_rst::gui::run_gui()
    }

    #[cfg(not(feature = "gui"))]
    {
        eprintln!("GUI not available. Use markitdown-cli for command-line mode.");
        std::process::exit(1);
    }
}
