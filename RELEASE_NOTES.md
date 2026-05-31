## MDrust v1.2.1 — Critical Windows Crash Fix & Renderer Fallback

This hotfix resolves a critical startup crash on Windows where the application would silently exit without showing any error message.

### Bug Fixes

- **Fixed silent crash on Windows when GPU initialization fails** — Previously, if `eframe::run_native()` returned an error (e.g., wgpu/GPU failure), the error was silently lost because there was no console (`windows_subsystem = "windows"`) and `tracing::error!` had no effect without the `logs` feature. Now, all eframe errors are shown in a Windows MessageBox with troubleshooting tips.
- **Added automatic renderer fallback (wgpu → glow)** — If the default wgpu renderer (DirectX 11/12) fails to initialize, MDrust now automatically retries with the glow renderer (OpenGL 3.0+). This covers systems with outdated GPU drivers, incompatible graphics hardware, or virtual machines without GPU passthrough.
- **Fixed output directory name** — Changed `markitdown-output` to `mdrust-output` (leftover from rename).
- **Improved panic messages** — Better `.expect()` messages for runtime creation failures.

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

# CLI: OCR with Russian + English
mdrust-cli convert scan.png --ocr-langs eng+rus
```

---

**Full Changelog**: https://github.com/AlexZander85/MDrust/compare/v1.2.0...v1.2.1
