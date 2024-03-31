use crate::gui::canvas::model::Canvas;
use crate::gui::editor::model::Editor;
use crate::interpreter::event::InputEvent;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::state::canvas::CanvasState;
use crate::interpreter::state::object::Point;
use eframe::egui::*;
use std::collections::HashSet;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct App {
    pub interpreter: Arc<Mutex<Interpreter>>,
    pub canvas: Arc<Mutex<Canvas>>,
    pub editor: Editor,
    pub input_sender: mpsc::Sender<InputEvent>,
    pub current_keys: HashSet<String>,
    pub is_running: Arc<Mutex<bool>>,
}

impl App {
    pub const EDITOR_WIDTH: f32 = 480.0;
    pub const CONSOLE_HEIGHT: f32 = 160.0;

    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (input_sender, input_receiver) = mpsc::channel::<InputEvent>();
        let mut interpreter = Interpreter::new();
        interpreter.bind_input_receiver(input_receiver);
        let canvas_size = vec2(CanvasState::DEFAULT_WIDTH, CanvasState::DEFAULT_HEIGHT);
        App {
            interpreter: Arc::from(Mutex::from(interpreter)),
            canvas: Arc::from(Mutex::from(Canvas::new(canvas_size))),
            editor: Editor::new(FontId::monospace(16.0)),
            input_sender,
            is_running: Arc::from(Mutex::from(false)),
            current_keys: HashSet::new(),
        }
    }

    pub fn run_code(&mut self, ctx: &Context) {
        self.current_keys.clear();

        // Set up a background thread to run interpreter independent of the UI.
        let interpreter_mutex = self.interpreter.clone();
        let canvas_mutex = self.canvas.clone();
        let ctx_mutex = Arc::from(Mutex::from(ctx.clone())).clone();
        let is_running_mutex = self.is_running.clone();
        let code = self.editor.current_code().to_string();
        thread::spawn(move || {
            let mut interpreter = interpreter_mutex.lock().unwrap();

            // Clear any events in the channel so stale key presses do not immediatley trigger.
            interpreter.clear_input_events();
            interpreter.bind_ui_handler(canvas_mutex, ctx_mutex);
            interpreter.interpret_main(&code);
            interpreter.clear_ui_handler();

            // Signal program no longer running.
            let mut is_running = is_running_mutex.lock().unwrap();
            *is_running = false;
        });

        // Signal program started running.
        let mut is_running = self.is_running.lock().unwrap();
        *is_running = true;
    }

    pub fn interrupt_code(&mut self) {
        let _ = self.input_sender.send(InputEvent::Interrupt);
    }

    pub fn reset_state(&mut self) {
        let is_running = *self.is_running.lock().unwrap();
        if is_running {
            println!("Can't reset when app is running");
            return;
        }

        // Delete all state from the interpreter.
        let mut interpreter = self.interpreter.lock().unwrap();
        interpreter.reset();

        // Create a new blank canvas.
        let new_canvas = Canvas::new(vec2(
            CanvasState::DEFAULT_WIDTH,
            CanvasState::DEFAULT_HEIGHT,
        ));
        let mut canvas = self.canvas.lock().unwrap();
        *canvas = new_canvas;
    }

    pub fn handle_key_commands(&mut self, input: &InputState, ctx: &Context) {
        if input.modifiers.command {
            if input.modifiers.shift {
                if input.key_pressed(Key::R) {
                    self.run_code(ctx);
                }
            } else {
                if input.key_pressed(Key::S) {
                    self.editor.save_current_file();
                } else if input.key_pressed(Key::N) {
                    self.editor.new_file();
                } else if input.key_pressed(Key::O) {
                    self.editor.open_file();
                } else if input.key_pressed(Key::W) {
                    self.editor.close_current_file();
                }
            }
        } else if input.modifiers.ctrl {
            if input.key_pressed(Key::C) {
                let _ = self.input_sender.send(InputEvent::Interrupt);
            }
        }
    }

    pub fn handle_keys(&mut self, input: &InputState) {
        let mut keys: HashSet<String> = input
            .keys_down
            .iter()
            .map(|key| key.name().to_string())
            .collect();
        if input.modifiers.shift {
            keys.insert(String::from("shift"));
        }

        // Keys that were just pressed.
        for key in keys.difference(&self.current_keys) {
            let _ = self
                .input_sender
                .send(InputEvent::KeyDown(key.clone().to_lowercase()));
        }

        // Keys that were just released.
        for key in self.current_keys.difference(&keys) {
            let _ = self
                .input_sender
                .send(InputEvent::KeyUp(key.clone().to_lowercase()));
        }

        // Set new keys to current keys.
        self.current_keys = keys;
    }

    pub fn handle_mouse(&self, input: &InputState) {
        if input.pointer.has_pointer() && input.pointer.any_click() {
            if let Some(mouse_pos) = input.pointer.interact_pos() {
                let canvas = self.canvas.lock().unwrap();

                // Only handle clicks that are on the app canvas area.
                if canvas.is_point_within(mouse_pos) {
                    let pos = canvas.from_canvas_coordinates(mouse_pos);
                    let point = Point::new(pos.x.round(), pos.y.round());
                    let _ = self.input_sender.send(InputEvent::Click(point));
                }
            }
        }
    }
}

impl eframe::App for App {
    // Build Main UI
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Left Side
        self.canvas_view(ctx);

        // Right Side
        self.code_editor_view(ctx);

        // Handle Mouse & Keyboard Events
        ctx.input(|input: &InputState| {
            self.handle_key_commands(input, ctx);
        });
        let is_focused = ctx.memory(|memory| memory.focus().is_some());
        if !is_focused {
            ctx.input(|input: &InputState| {
                self.handle_keys(input);
                self.handle_mouse(input);
            });
        }
    }
}
