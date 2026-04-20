Steam Clip Exporter
===

<img src="./assets/screenshot.png" width="800" alt="Screenshot">

### Motivation

Steam has a feature called 'Game Recording'.
It's a neat feature. You can record clips of your gameplay (automatically or manually with a shortcut) and export them to a file.

But there are some issues:

1. You can't export a clip to H265 encoding (on both Windows and Linux) ([#11849](https://github.com/ValveSoftware/steam-for-linux/issues/11849))
2. It takes a long time to re-encode a clip when you export it to H264 encoding even if it is encoded in H264 in the first place.

This tool is a workaround for these issues.

### How to build

#### Prerequisites

1. [Rust compiler](https://rustup.rs/)
2. [gtk-rs development libraries](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_linux.html)
3. `ffmpeg` in your `PATH`

### Build and run

```bash
git clone https://github.com/foriequal0/steam-clip-exporter.git
cd steam-clip-exporter
cargo run
```

If you prefer flatpak,

```bash
flatpak-builder --force-clean --user --install-deps-from=flathub --repo=repo --install builddir io.github.foriequal0.SteamClipExporter.yml
flatpak run io.github.foriequal0.SteamClipExporter
```

### TODO

* [ ] export progress bar or spinner
* [ ] show game title instead of app id
* [ ] configurations (steam root, window size, etc)
* [ ] handle more cases
    * [ ] background recording
    * [ ] unknown fields
* [ ] packaging
    * [ ] deb
    * [ ] AppImage
