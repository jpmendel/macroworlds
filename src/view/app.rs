use crate::interpreter::interpreter::Interpreter;
use crate::language::event::{InputEvent, UiEvent};
use crate::view::canvas::Canvas;
use eframe::egui::*;
use std::collections::HashSet;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct App {
    pub interpreter: Arc<Mutex<Interpreter>>,
    pub canvas: Arc<Mutex<Canvas>>,
    pub code: String,
    pub canvas_pos: Pos2,
    pub canvas_size: Vec2,
    pub input_sender: mpsc::Sender<InputEvent>,
    pub ui_receiver: Arc<Mutex<mpsc::Receiver<UiEvent>>>,
    pub is_running: Arc<Mutex<bool>>,
    pub key_buffer: HashSet<String>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (ui_sender, ui_receiver) = mpsc::channel::<UiEvent>();
        let (input_sender, input_receiver) = mpsc::channel::<InputEvent>();
        let interpreter = Interpreter::new(ui_sender, input_receiver);
        let canvas_pos = pos2(50.0, 50.0);
        let canvas_size = vec2(500.0, 400.0);
        App {
            interpreter: Arc::from(Mutex::from(interpreter)),
            code: String::new(),
            canvas_pos,
            canvas_size,
            canvas: Arc::from(Mutex::from(Canvas::new(canvas_pos, canvas_size))),
            input_sender,
            ui_receiver: Arc::from(Mutex::from(ui_receiver)),
            is_running: Arc::from(Mutex::from(false)),
            key_buffer: HashSet::new(),
        }
    }

    pub fn run_code(&mut self, ctx: &Context) {
        self.key_buffer.clear();

        // Set up a background thread to listen to UI events coming over the channel.
        let canvas_mutex = self.canvas.clone();
        let receiver_mutex = self.ui_receiver.clone();
        let ctx_mutex = Arc::from(Mutex::from(ctx.clone())).clone();
        thread::spawn(move || {
            let event_receiver = receiver_mutex.lock().unwrap();
            let timeout = Duration::from_secs(2);
            while let Ok(event) = event_receiver.recv_timeout(timeout) {
                if let UiEvent::Done = event {
                    break;
                }
                let mut canvas = canvas_mutex.lock().unwrap();
                let ctx = ctx_mutex.lock().unwrap();
                canvas.handle_ui_event(&ctx, event);
            }
            while event_receiver.try_recv().is_ok() {
                // Consume remaining events.
            }
        });

        // Set up a background thread to run interpreter and send any UI updates.
        let interpreter_mutex = self.interpreter.clone();
        let is_running_mutex = self.is_running.clone();
        let code = self.code.clone();
        thread::spawn(move || {
            let mut interpreter = interpreter_mutex.lock().unwrap();
            while interpreter.input_receiver.try_recv().is_ok() {
                // Consume remaining events.
            }
            let _ = interpreter.interpret(&code);
            let mut is_running = is_running_mutex.lock().unwrap();
            *is_running = false;
        });

        *self.is_running.lock().unwrap() = true;
    }

    pub fn interrupt_code(&mut self) {
        self.input_sender.send(InputEvent::Interrupt).unwrap_or(());
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        let size = ctx.input(|i| i.viewport().outer_rect).unwrap();

        // Canvas and Console
        SidePanel::left("left")
            .frame(Frame::none())
            .exact_width(size.width() * 0.6)
            .resizable(false)
            .show(ctx, |ui: &mut Ui| {
                let canvas = self.canvas.lock().unwrap();

                // Output Console
                TopBottomPanel::bottom("bottom_left")
                    .frame(Frame::none())
                    .exact_height(size.height() * 0.2)
                    .resizable(false)
                    .show_inside(ui, |ui: &mut Ui| {
                        ui.add_space(6.0);

                        ui.horizontal(|ui: &mut Ui| {
                            ui.add_space(6.0);
                            let print_output = RichText::new(canvas.text.clone())
                                .font(FontId::proportional(16.0))
                                .color(Color32::from_gray(255));

                            let print_output_label = Label::new(print_output);
                            ui.add(print_output_label);
                        });
                    });

                ui.add_space(10.0);

                // Title
                ui.horizontal(|ui: &mut Ui| {
                    ui.add_space(10.0);
                    // Title text.
                    let title = RichText::new(String::from("MicroWorlds.rs"))
                        .font(FontId::proportional(18.0))
                        .color(Color32::from_gray(255));
                    let title_label = Label::new(title);
                    ui.add(title_label);
                });

                // Blank Canvas
                let painter = ui.painter();
                let rect = Rect::from_x_y_ranges(
                    Rangef::new(self.canvas_pos.x, self.canvas_pos.x + self.canvas_size.x),
                    Rangef::new(self.canvas_pos.y, self.canvas_pos.y + self.canvas_size.y),
                );
                painter.rect_filled(rect, Rounding::same(0.0), Color32::from_gray(255));

                // Lines
                let content_painter = ui.painter_at(rect);
                for line in &canvas.lines {
                    content_painter.line_segment(
                        [line.start.clone(), line.end.clone()],
                        Stroke::new(3.0, line.color),
                    );
                }

                // Turtles
                for (_, turtle) in &canvas.turtles {
                    content_painter.circle_filled(turtle.pos, 5.0, turtle.color);
                }
            });

        // Code Editor
        SidePanel::right("right")
            .frame(Frame::none())
            .exact_width(size.width() * 0.4)
            .resizable(false)
            .show(ctx, |ui: &mut Ui| {
                ui.vertical(|ui: &mut Ui| {
                    // Buttons
                    TopBottomPanel::bottom("bottom_right")
                        .frame(Frame::none())
                        .exact_height(size.height() * 0.1)
                        .resizable(false)
                        .show_inside(ui, |ui: &mut Ui| {
                            let is_running = *self.is_running.lock().unwrap();
                            let button_text = if is_running {
                                String::from("Stop")
                            } else {
                                String::from("Run Code")
                            };
                            let button_label = RichText::new(button_text)
                                .font(FontId::proportional(16.0))
                                .color(Color32::from_gray(255));
                            let button = Button::new(button_label);
                            let button_ref = ui
                                .add_sized(vec2(size.width() * 0.4, ui.available_height()), button);
                            if button_ref.clicked() {
                                if is_running {
                                    self.interrupt_code();
                                } else {
                                    self.run_code(ctx);
                                }
                            }
                        });

                    ui.add_space(10.0);

                    // Title
                    ui.horizontal(|ui: &mut Ui| {
                        ui.add_space(10.0);
                        let title = RichText::new(String::from("Editor"))
                            .font(FontId::proportional(18.0))
                            .color(Color32::from_gray(255));
                        let title_label = Label::new(title);
                        ui.add(title_label);
                    });

                    ui.add_space(10.0);

                    // Text Area
                    ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                        let text_field = TextEdit::multiline(&mut self.code)
                            .code_editor()
                            .font(FontId::monospace(16.0));
                        ui.add_sized(
                            vec2(size.width() * 0.396, ui.available_height()),
                            text_field,
                        );
                    });
                });
            });

        // Handle Keyboard Events
        let is_focused = ctx.memory(|memory| memory.focus().is_some());
        if !is_focused {
            ctx.input(|input| {
                let keys: HashSet<String> = input
                    .keys_down
                    .iter()
                    .map(|key| key.name().to_string())
                    .collect();
                let diff = self.key_buffer.difference(&keys);
                for key in diff {
                    let _ = self
                        .input_sender
                        .send(InputEvent::Key(key.clone().to_lowercase()));
                }
                self.key_buffer = keys;
            });
        }
    }
}
