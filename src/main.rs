use eframe::egui::{self, Align2, Key, KeyboardShortcut, Modifiers};
use std::{fs, path::PathBuf};

struct Editor {
    text: String,
    path: Option<PathBuf>,
    dirty: bool,
    wrap: bool,
    status: String,
    line_col: (usize, usize),
    confirm_quit: bool,
    force_quit: bool,
    last_title: String,
}

impl Editor {
    fn new() -> Self {
        let mut ed = Self {
            text: String::new(),
            path: None,
            dirty: false,
            wrap: true,
            status: "Ready".into(),
            line_col: (1, 1),
            confirm_quit: false,
            force_quit: false,
            last_title: String::new(),
        };
        // `rusty-pad some/file.txt` opens that file
        if let Some(arg) = std::env::args().nth(1) {
            ed.load(PathBuf::from(arg));
        }
        ed
    }

    fn title(&self) -> String {
        let name = self
            .path
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "Untitled".into());
        format!("{}{} — Rusty Pad", if self.dirty { "● " } else { "" }, name)
    }

    fn new_file(&mut self) {
        self.text.clear();
        self.path = None;
        self.dirty = false;
        self.status = "New file".into();
    }

    fn load(&mut self, p: PathBuf) {
        match fs::read_to_string(&p) {
            Ok(s) => {
                self.text = s;
                self.status = format!("Opened {}", p.display());
                self.path = Some(p);
                self.dirty = false;
            }
            Err(e) => self.status = format!("Open failed: {e}"),
        }
    }

    fn open(&mut self) {
        if let Some(p) = rfd::FileDialog::new().pick_file() {
            self.load(p);
        }
    }

    fn save(&mut self) {
        match self.path.clone() {
            Some(p) => self.write(p),
            None => self.save_as(),
        }
    }

    fn save_as(&mut self) {
        if let Some(p) = rfd::FileDialog::new().save_file() {
            self.write(p);
        }
    }

    fn write(&mut self, p: PathBuf) {
        match fs::write(&p, &self.text) {
            Ok(()) => {
                self.status = format!("Saved {}", p.display());
                self.path = Some(p);
                self.dirty = false;
            }
            Err(e) => self.status = format!("Save failed: {e}"),
        }
    }
}

impl eframe::App for Editor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ---- keyboard shortcuts (Save As is checked before Save) ----
        let cmd = Modifiers::COMMAND;
        if ctx.input_mut(|i| i.consume_shortcut(&KeyboardShortcut::new(cmd | Modifiers::SHIFT, Key::S))) {
            self.save_as();
        } else if ctx.input_mut(|i| i.consume_shortcut(&KeyboardShortcut::new(cmd, Key::S))) {
            self.save();
        }
        if ctx.input_mut(|i| i.consume_shortcut(&KeyboardShortcut::new(cmd, Key::O))) {
            self.open();
        }
        if ctx.input_mut(|i| i.consume_shortcut(&KeyboardShortcut::new(cmd, Key::N))) {
            self.new_file();
        }

        // ---- window title ----
        let title = self.title();
        if title != self.last_title {
            ctx.send_viewport_cmd(egui::ViewportCommand::Title(title.clone()));
            self.last_title = title;
        }

        // ---- unsaved-changes guard on close ----
        if ctx.input(|i| i.viewport().close_requested()) && self.dirty && !self.force_quit {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.confirm_quit = true;
        }
        if self.confirm_quit {
            egui::Window::new("Unsaved changes")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label("You have unsaved changes.");
                    ui.horizontal(|ui| {
                        if ui.button("Save & quit").clicked() {
                            self.save();
                            if !self.dirty {
                                self.force_quit = true;
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                            self.confirm_quit = false;
                        }
                        if ui.button("Discard").clicked() {
                            self.force_quit = true;
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        if ui.button("Cancel").clicked() {
                            self.confirm_quit = false;
                        }
                    });
                });
        }

        // ---- menu bar ----
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New        Ctrl+N").clicked() {
                        self.new_file();
                        ui.close_menu();
                    }
                    if ui.button("Open…      Ctrl+O").clicked() {
                        self.open();
                        ui.close_menu();
                    }
                    if ui.button("Save       Ctrl+S").clicked() {
                        self.save();
                        ui.close_menu();
                    }
                    if ui.button("Save As…   Ctrl+Shift+S").clicked() {
                        self.save_as();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        ui.close_menu();
                    }
                });
                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.wrap, "Word wrap");
                });
            });
        });

        // ---- status bar ----
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Ln {}, Col {}", self.line_col.0, self.line_col.1));
                });
            });
        });

        // ---- editor area ----
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
                let width = if self.wrap { ui.available_width() } else { f32::INFINITY };
                let out = egui::TextEdit::multiline(&mut self.text)
                    .font(egui::TextStyle::Monospace)
                    .desired_width(width)
                    .min_size(ui.available_size())
                    .lock_focus(true)
                    .show(ui);

                if out.response.changed() {
                    self.dirty = true;
                }
                if let Some(range) = out.cursor_range {
                    let idx = range.primary.ccursor.index;
                    let before: String = self.text.chars().take(idx).collect();
                    let line = before.matches('\n').count() + 1;
                    let col = before.chars().rev().take_while(|c| *c != '\n').count() + 1;
                    self.line_col = (line, col);
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_title("Rusty Pad")
            .with_app_id("rusty-pad"),
        ..Default::default()
    };
    eframe::run_native(
        "Rusty Pad",
        options,
        Box::new(|_cc| Ok(Box::new(Editor::new()))),
    )
}
