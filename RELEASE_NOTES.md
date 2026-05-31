## MDrust v1.2.2 — Critical Font Crash Fix

This hotfix resolves a fatal startup crash caused by corrupted font files.

### Bug Fixes

- **Fixed fatal crash: "Error parsing Inter-Bold TTF/OTF font file: InvalidFont"** — The `Inter-Regular.ttf` and `Inter-Bold.ttf` files in the `assets/fonts/` directory were accidentally HTML pages (downloaded from a wrong URL) instead of real TTF font binaries. egui's font parser would panic when trying to parse them, causing the application to crash immediately on startup.
- **Added font validation** — Font files are now validated for TTF/OTF magic bytes before being passed to egui. If a font file is corrupted or invalid, it is silently skipped with a warning instead of causing a panic. The application will start with egui's built-in fallback fonts.
- **Fixed output directory name** — Changed `markitdown-output` to `mdrust-output` (leftover from project rename).
- **Added GPU error fallback** — If the wgpu renderer fails, MDrust now automatically retries with the glow (OpenGL) renderer.
- **Added error dialog on Windows** — eframe startup errors are now shown in a Windows MessageBox instead of being silently lost.

### Downloads

| File | Edition | OS | Arch |
|------|---------|----|------|
| `mdrust-full-linux-x64.tar.gz` | Full (GUI + OCR + Preview) | Linux | x86_64 |
| `mdrust-full-macos-x64.tar.gz` | Full | macOS | x86_64 |
| `mdrust-full-windows-x64.exe` | Full | Windows | x86_64 |
| `mdrust-light-linux-x64.tar.gz` | Light (GUI, no OCR) | Linux | x86_64 |
| `mdrust-light-macos-x64.tar.gz` | Light | macOS | x86_64 |
| `mdrust-light-windows-x64.exe` | Light | Windows | x86_64 |
| `mdrust-cli-linux-x64.tar.gz` | CLI-only (OCR) | Linux | x86_64 |
| `mdrust-cli-macos-x64.tar.gz` | CLI-only | macOS | x86_64 |
| `mdrust-cli-windows-x64.exe` | CLI-only | Windows | x86_64 |

### Quick Start

```bash
# GUI mode
./mdrust

# CLI: single file
mdrust-cli convert document.pdf

# CLI: batch with 8 threads
mdrust-cli batch ./docs --threads 8 --output ./markdown
```

---

**Full Changelog**: https://github.com/AlexZander85/MDrust/compare/v1.2.1...v1.2.2
