## MDrust v1.2.3 — Icon Fix, Chinese Font Fix, Theme Fix, OCR Message Fix

This release fixes multiple UI issues reported after v1.2.2.

### Bug Fixes

- **Fixed squished app icon** — The source icon was 1536×1024 (not square), causing egui to display it compressed horizontally. The icon is now properly cropped to square (1024×1024 center crop) before resizing to 256×256. The Windows `.ico` file now includes 16/32/48/256 sizes instead of just 16×16.
- **Fixed Chinese characters not displaying** — The previous `NotoSansSC-Regular.ttf` was a variable font (5.2MB with `fvar`/`gvar` tables) that egui's font parser couldn't handle. Replaced with a proper static TrueType font (10.5MB, 30,890 glyphs) from Google Fonts CDN. Chinese text now renders correctly in the language selector, OCR checkboxes, and throughout the UI.
- **Fixed dark theme text invisible on first launch** — Sidebar labels (Output Dir, Threads, etc.) were nearly invisible on dark theme at startup, but fine after toggling theme. Cause: `Theme::apply()` was called before `fonts::install()`, but `ctx.set_fonts()` resets the style/visuals. Fixed by installing fonts first, then applying theme.
- **Clarified Tesseract OCR message** — The status bar now explains that language data (tessdata) is embedded in MDrust, but the Tesseract binary must be installed separately. This is by design — embedding a C++ binary is not feasible, but the language packs (which are what users usually struggle with) are built-in.

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

**Full Changelog**: https://github.com/AlexZander85/MDrust/compare/v1.2.2...v1.2.3
