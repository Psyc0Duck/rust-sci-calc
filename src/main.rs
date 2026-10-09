//! Wissenschaftlicher Taschenrechner mit grafischer Oberfläche (egui).

#![forbid(unsafe_code)]
// Unter Windows kein zusätzliches Konsolenfenster im Release-Build öffnen.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod parser;

use eframe::egui;
use parser::{AngleMode, MAX_INPUT_LEN, evaluate, format_number};

/// Maximale Anzahl gespeicherter Verlaufseinträge.
const MAX_HISTORY: usize = 50;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Taschenrechner")
            .with_inner_size([620.0, 640.0])
            .with_min_inner_size([480.0, 520.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Taschenrechner",
        options,
        Box::new(|_cc| Ok(Box::new(Calculator::default()))),
    )
}

struct Calculator {
    input: String,
    result: Option<Result<f64, String>>,
    ans: f64,
    memory: f64,
    mode: AngleMode,
    history: Vec<(String, String)>,
    /// Nach Tastenklicks den Textcursor ans Ende der Eingabe setzen.
    cursor_to_end: bool,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            input: String::new(),
            result: None,
            ans: 0.0,
            memory: 0.0,
            mode: AngleMode::Deg,
            history: Vec::new(),
            cursor_to_end: false,
        }
    }
}

/// Was eine Taste beim Drücken bewirkt.
#[derive(Clone, Copy)]
enum Action {
    Insert(&'static str),
    Clear,
    Backspace,
    Equals,
    ToggleAngle,
    MemoryAdd,
    MemoryRecall,
    MemoryClear,
}

struct Key {
    label: &'static str,
    action: Action,
    kind: KeyKind,
}

#[derive(Clone, Copy, PartialEq)]
enum KeyKind {
    Digit,
    Operator,
    Function,
    Control,
    Equal,
}

const fn key(label: &'static str, action: Action, kind: KeyKind) -> Key {
    Key {
        label,
        action,
        kind,
    }
}

use Action::*;
use KeyKind::*;

const KEYS: [[Key; 5]; 9] = [
    [
        key("DEG/RAD", ToggleAngle, Control),
        key("MC", MemoryClear, Control),
        key("MR", MemoryRecall, Control),
        key("M+", MemoryAdd, Control),
        key("Ans", Insert("ans"), Control),
    ],
    [
        key("sin", Insert("sin("), Function),
        key("cos", Insert("cos("), Function),
        key("tan", Insert("tan("), Function),
        key("π", Insert("π"), Function),
        key("e", Insert("e"), Function),
    ],
    [
        key("asin", Insert("asin("), Function),
        key("acos", Insert("acos("), Function),
        key("atan", Insert("atan("), Function),
        key("ln", Insert("ln("), Function),
        key("log", Insert("log("), Function),
    ],
    [
        key("x²", Insert("^2"), Function),
        key("x^y", Insert("^"), Function),
        key("√", Insert("√("), Function),
        key("n!", Insert("!"), Function),
        key("1/x", Insert("^-1"), Function),
    ],
    [
        key("(", Insert("("), Operator),
        key(")", Insert(")"), Operator),
        key("mod", Insert("%"), Operator),
        key("C", Clear, Control),
        key("DEL", Backspace, Control),
    ],
    [
        key("7", Insert("7"), Digit),
        key("8", Insert("8"), Digit),
        key("9", Insert("9"), Digit),
        key("÷", Insert("÷"), Operator),
        key("abs", Insert("abs("), Function),
    ],
    [
        key("4", Insert("4"), Digit),
        key("5", Insert("5"), Digit),
        key("6", Insert("6"), Digit),
        key("×", Insert("×"), Operator),
        key("cbrt", Insert("cbrt("), Function),
    ],
    [
        key("1", Insert("1"), Digit),
        key("2", Insert("2"), Digit),
        key("3", Insert("3"), Digit),
        key("−", Insert("-"), Operator),
        key("EXP", Insert("e"), Function),
    ],
    [
        key("0", Insert("0"), Digit),
        key(",", Insert(","), Digit),
        key("=", Equals, Equal),
        key("+", Insert("+"), Operator),
        key("round", Insert("round("), Function),
    ],
];

impl Calculator {
    fn insert(&mut self, text: &str) {
        if self.input.chars().count() + text.chars().count() <= MAX_INPUT_LEN {
            self.input.push_str(text);
        }
        self.cursor_to_end = true;
    }

    fn apply(&mut self, action: Action) {
        match action {
            Insert(text) => {
                // Nach einem Ergebnis mit einem Operator direkt weiterrechnen.
                if self.input.is_empty()
                    && matches!(self.result, Some(Ok(_)))
                    && matches!(text, "+" | "-" | "×" | "÷" | "^" | "^2" | "^-1" | "!" | "%")
                {
                    self.insert("ans");
                }
                self.insert(text);
            }
            Clear => {
                self.input.clear();
                self.result = None;
            }
            Backspace => {
                self.input.pop();
                self.cursor_to_end = true;
            }
            Equals => self.calculate(),
            ToggleAngle => {
                self.mode = match self.mode {
                    AngleMode::Deg => AngleMode::Rad,
                    AngleMode::Rad => AngleMode::Deg,
                };
            }
            MemoryAdd => {
                if let Some(Ok(v)) = self.result {
                    self.memory += v;
                }
            }
            MemoryRecall => {
                let text = format!("({})", format_number(self.memory));
                self.insert(&text);
            }
            MemoryClear => self.memory = 0.0,
        }
    }

    fn calculate(&mut self) {
        let expr = self.input.trim().to_string();
        if expr.is_empty() {
            return;
        }
        match evaluate(&expr, self.mode, self.ans) {
            Ok(v) => {
                self.ans = v;
                self.result = Some(Ok(v));
                if self.history.len() >= MAX_HISTORY {
                    self.history.remove(0);
                }
                self.history.push((expr, format_number(v)));
                self.input.clear();
            }
            Err(e) => self.result = Some(Err(e.to_string())),
        }
    }

    fn display(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style())
            .inner_margin(10.0)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    let mode = match self.mode {
                        AngleMode::Deg => "DEG",
                        AngleMode::Rad => "RAD",
                    };
                    ui.label(egui::RichText::new(mode).small().strong());
                    if self.memory != 0.0 {
                        ui.label(egui::RichText::new("M").small().strong());
                    }
                });
                let edit = egui::TextEdit::singleline(&mut self.input)
                    .font(egui::FontId::proportional(22.0))
                    .hint_text("z. B. 2·sin(30) + √(16)")
                    .char_limit(MAX_INPUT_LEN)
                    .desired_width(f32::INFINITY);
                let mut output = edit.show(ui);
                if self.cursor_to_end {
                    let end = egui::text::CCursor::new(self.input.chars().count());
                    output
                        .state
                        .cursor
                        .set_char_range(Some(egui::text::CCursorRange::one(end)));
                    output.state.store(ui.ctx(), output.response.id);
                    self.cursor_to_end = false;
                }
                let response = output.response;
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.calculate();
                }
                // Eingabefeld behält den Fokus, damit man direkt tippen kann.
                if ui.ctx().memory(|m| m.focused().is_none()) {
                    response.request_focus();
                }
                let (text, color) = match &self.result {
                    Some(Ok(v)) => (
                        format!("= {}", format_number(*v)),
                        ui.visuals().strong_text_color(),
                    ),
                    Some(Err(e)) => (e.clone(), ui.visuals().error_fg_color),
                    None => (" ".into(), ui.visuals().text_color()),
                };
                let size = egui::vec2(ui.available_width(), 40.0);
                let layout = egui::Layout::right_to_left(egui::Align::Center);
                ui.allocate_ui_with_layout(size, layout, |ui| {
                    ui.label(egui::RichText::new(text).size(26.0).color(color));
                });
            });
    }

    fn keypad(&mut self, ui: &mut egui::Ui) {
        let spacing = 6.0;
        let cols = 5.0;
        let rows = KEYS.len() as f32;
        let w = ((ui.available_width() - spacing * (cols - 1.0)) / cols).max(40.0);
        let h = ((ui.available_height() - spacing * (rows - 1.0)) / rows).clamp(30.0, 60.0);
        let dark = ui.visuals().dark_mode;
        let mut pressed = None;
        egui::Grid::new("keypad")
            .spacing([spacing, spacing])
            .show(ui, |ui| {
                for row in &KEYS {
                    for k in row {
                        let mut text = egui::RichText::new(k.label).size(18.0);
                        if k.kind == Equal {
                            text = text.color(egui::Color32::WHITE).strong();
                        }
                        let button = egui::Button::new(text).fill(key_color(k.kind, dark));
                        if ui.add_sized([w, h], button).clicked() {
                            pressed = Some(k.action);
                        }
                    }
                    ui.end_row();
                }
            });
        if let Some(action) = pressed {
            self.apply(action);
            ui.ctx().request_repaint();
        }
    }

    fn history_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Verlauf");
        ui.separator();
        let mut picked = None;
        egui::ScrollArea::vertical()
            .max_height(ui.available_height() - 30.0)
            .show(ui, |ui| {
                for (expr, res) in self.history.iter().rev() {
                    let r = ui
                        .add(
                            egui::Label::new(format!("{expr}\n= {res}"))
                                .sense(egui::Sense::click()),
                        )
                        .on_hover_text("Klicken, um das Ergebnis einzufügen");
                    if r.clicked() {
                        picked = Some(res.clone());
                    }
                    ui.separator();
                }
            });
        if let Some(text) = picked {
            self.insert(&text);
        }
        if !self.history.is_empty() && ui.button("Verlauf löschen").clicked() {
            self.history.clear();
        }
    }
}

fn key_color(kind: KeyKind, dark: bool) -> egui::Color32 {
    use egui::Color32 as C;
    match (kind, dark) {
        (Digit, true) => C::from_rgb(64, 64, 70),
        (Digit, false) => C::from_rgb(252, 252, 254),
        (Operator, true) => C::from_rgb(50, 56, 72),
        (Operator, false) => C::from_rgb(222, 230, 244),
        (Function, true) => C::from_rgb(40, 44, 52),
        (Function, false) => C::from_rgb(236, 238, 242),
        (Control, true) => C::from_rgb(76, 52, 52),
        (Control, false) => C::from_rgb(246, 224, 220),
        (Equal, _) => C::from_rgb(30, 110, 200),
    }
}

impl eframe::App for Calculator {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.apply(Clear);
        }
        egui::Panel::right("history")
            .resizable(true)
            .default_size(170.0)
            .show(ui, |ui| self.history_panel(ui));
        egui::CentralPanel::default_margins().show(ui, |ui| {
            self.display(ui);
            ui.add_space(8.0);
            self.keypad(ui);
        });
    }
}
