# Rusty Pad

A simple, fast text editor for Linux, written entirely in Rust with [egui](https://github.com/emilk/egui). It runs natively on both Wayland and X11 and is developed and tested on Ubuntu 26.10.

## Features

- Create, open, save and save-as plain text files
- Native file dialogs (via the desktop portal)
- Word wrap toggle
- Line and column indicator in the status bar
- Unsaved-changes marker in the title bar and a prompt before quitting
- Open a file directly from the terminal: `rusty-pad notes.txt`

## Keyboard shortcuts

| Shortcut | Action |
|---|---|
| Ctrl+N | New file |
| Ctrl+O | Open file |
| Ctrl+S | Save |
| Ctrl+Shift+S | Save as |

## Install

### Option 1: Download the prebuilt binary

1. Go to the [Releases page](../../releases) and download the latest `rusty-pad` file.
2. Make it executable and run it:

```bash
chmod +x rusty-pad
./rusty-pad
```

To install it for your user so you can run `rusty-pad` from any terminal:

```bash
mkdir -p ~/.local/bin
mv rusty-pad ~/.local/bin/
```

> The prebuilt binary is built on Ubuntu 26.10. It may not run on much older distributions because of differences in system libraries. If it doesn't start, build from source (below).

### Option 2: Build from source

Install the build dependencies and Rust:

```bash
sudo apt install build-essential pkg-config libxkbcommon-dev libwayland-dev libx11-dev libgl1-mesa-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then clone and build:

```bash
git clone https://github.com/LunaVibeCoding/RustyPad.git
cd RustyPad
cargo build --release
```

The binary will be at `target/release/rusty-pad`. To install it to `~/.cargo/bin`:

```bash
cargo install --path .
```

## Add it to your app launcher

Create `~/.local/share/applications/rusty-pad.desktop`:

```ini
[Desktop Entry]
Name=Rusty Pad
Exec=/home/YOURNAME/.local/bin/rusty-pad %f
Type=Application
Categories=Utility;TextEditor;
Terminal=false
```

Replace `YOURNAME` with your username (and adjust the path if you installed elsewhere). Log out and back in if it doesn't appear right away.

## Roadmap

- [ ] Find and replace
- [ ] Syntax highlighting
- [ ] Tabs
- [ ] Theme and font size settings

## License

Add a license of your choice (for example MIT) as a `LICENSE` file.
