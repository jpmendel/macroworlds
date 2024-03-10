use crate::interpreter::event::{InputEvent, UiEvent};
use crate::interpreter::interpreter::Interpreter;
use crate::state::state::State;
use crate::view::canvas::CanvasView;
use crate::view::object::ObjectView;
use eframe::egui::*;
use rfd::FileDialog;
use std::collections::HashSet;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct App {
    pub interpreter: Arc<Mutex<Interpreter>>,
    pub canvas: Arc<Mutex<CanvasView>>,
    pub code: String,
    pub input_sender: mpsc::Sender<InputEvent>,
    pub ui_receiver: Arc<Mutex<mpsc::Receiver<UiEvent>>>,
    pub is_running: Arc<Mutex<bool>>,
    pub key_buffer: HashSet<String>,
}

impl App {
    const EDITOR_WIDTH: f32 = 480.0;
    const CONSOLE_HEIGHT: f32 = 160.0;

    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (ui_sender, ui_receiver) = mpsc::channel::<UiEvent>();
        let (input_sender, input_receiver) = mpsc::channel::<InputEvent>();
        let interpreter = Interpreter::new(ui_sender, input_receiver);
        let canvas_size = vec2(
            State::DEFAULT_CANVAS_WIDTH.clone(),
            State::DEFAULT_CANVAS_HEIGHT.clone(),
        );
        App {
            interpreter: Arc::from(Mutex::from(interpreter)),
            code: String::new(),
            canvas: Arc::from(Mutex::from(CanvasView::with(canvas_size))),
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
            let _ = interpreter.interpret_main(&code);
            let mut is_running = is_running_mutex.lock().unwrap();
            *is_running = false;
        });

        *self.is_running.lock().unwrap() = true;
    }

    pub fn interrupt_code(&mut self) {
        self.input_sender.send(InputEvent::Interrupt).unwrap_or(());
    }

    pub fn open_file(&mut self) {
        let file_name = FileDialog::new()
            .add_filter("logo", &["txt", "logo"])
            .set_directory(".")
            .pick_file();
        if let Some(file_name) = file_name {
            if let Ok(mut file) = File::open(file_name) {
                let mut contents = String::new();
                if let Err(err) = file.read_to_string(&mut contents) {
                    println!("Failed to load file: {}", err);
                } else {
                    self.code = contents;
                }
            }
        }
    }

    pub fn save_file(&self) {
        let file_name = FileDialog::new()
            .set_file_name("untitled.logo")
            .set_directory(".")
            .save_file();
        if let Some(file_name) = file_name {
            if let Ok(mut file) = File::create(file_name) {
                let code = self.code.clone();
                if let Err(err) = file.write_all(code.as_bytes()) {
                    println!("Failed to save file: {}", err);
                }
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let size = ctx.input(|i| i.viewport().outer_rect).unwrap();
        let main_frame_width = size.width() - App::EDITOR_WIDTH;
        let main_frame_height = size.height() - App::CONSOLE_HEIGHT;

        // Canvas and Console
        SidePanel::left("left")
            .frame(Frame::default().fill(Color32::from_gray(20)))
            .exact_width(main_frame_width)
            .resizable(false)
            .show(ctx, |ui: &mut Ui| {
                let mut canvas = self.canvas.lock().unwrap();

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
                let canvas_pos = pos2(
                    main_frame_width / 2.0 - canvas.size.x / 2.0,
                    main_frame_height / 2.0 - canvas.size.y / 2.0,
                );
                canvas.pos = canvas_pos;
                let rect = Rect::from_x_y_ranges(
                    Rangef::new(canvas_pos.x, canvas_pos.x + canvas.size.x),
                    Rangef::new(canvas_pos.y, canvas_pos.y + canvas.size.y),
                );
                painter.rect_filled(rect, Rounding::same(0.0), canvas.bg_color);

                // Lines
                let content_painter = ui.painter_at(rect);
                for line in &canvas.lines {
                    content_painter.line_segment(
                        [
                            canvas.to_canvas_coordinates(line.start),
                            canvas.to_canvas_coordinates(line.end),
                        ],
                        Stroke::new(3.0, line.color),
                    );
                }

                // Turtles and Text
                for (_, obj) in &canvas.objects {
                    match obj {
                        ObjectView::Turtle(turtle) => {
                            if turtle.is_visible {
                                let shape = canvas.shape_for_turtle(turtle);
                                content_painter.add(shape);
                            }
                        }
                        ObjectView::Text(text) => {
                            if text.is_visible {
                                content_painter.text(
                                    canvas.to_canvas_coordinates(text.pos),
                                    Align2::CENTER_CENTER,
                                    text.text.to_string(),
                                    FontId::proportional(text.font_size),
                                    text.color,
                                );
                            }
                        }
                    }
                }

                // Output Console
                TopBottomPanel::bottom("bottom_left")
                    .frame(Frame::default().fill(Color32::from_gray(40)))
                    .exact_height(App::CONSOLE_HEIGHT)
                    .resizable(false)
                    .show_inside(ui, |ui: &mut Ui| {
                        ui.add_space(6.0);

                        ui.horizontal(|ui: &mut Ui| {
                            ui.add_space(6.0);
                            let print_output = RichText::new(canvas.console_text.clone())
                                .font(FontId::proportional(16.0))
                                .color(Color32::from_gray(255));
                            let print_output_label = Label::new(print_output);
                            ui.add(print_output_label);
                        });
                    });
            });

        // Code Editor
        SidePanel::right("right")
            .frame(Frame::default().fill(Color32::from_gray(20)))
            .exact_width(App::EDITOR_WIDTH)
            .resizable(false)
            .show(ctx, |ui: &mut Ui| {
                // Save/Load
                TopBottomPanel::top("top_right")
                    .frame(Frame::default().fill(Color32::from_gray(20)))
                    .resizable(false)
                    .show_inside(ui, |ui: &mut Ui| {
                        ui.add_space(10.0);

                        ui.horizontal(|ui: &mut Ui| {
                            ui.add_space(10.0);
                            let title = RichText::new(String::from("Editor"))
                                .font(FontId::proportional(18.0))
                                .color(Color32::from_gray(255));
                            let title_label = Label::new(title);
                            ui.add(title_label);

                            let open_button_label = RichText::new(String::from("Open"))
                                .font(FontId::proportional(14.0))
                                .color(Color32::from_gray(255));
                            let open_button = Button::new(open_button_label);
                            let open_button_ref = ui.add_sized(vec2(60.0, 20.0), open_button);
                            if open_button_ref.clicked() {
                                self.open_file();
                            }

                            let save_button_label = RichText::new(String::from("Save"))
                                .font(FontId::proportional(14.0))
                                .color(Color32::from_gray(255));
                            let save_button = Button::new(save_button_label);
                            let save_button_ref = ui.add_sized(vec2(60.0, 20.0), save_button);
                            if save_button_ref.clicked() {
                                self.save_file()
                            }

                            let docs_button_label = RichText::new(String::from("Docs"))
                                .font(FontId::proportional(14.0))
                                .color(Color32::from_gray(255));
                            let docs_button = Button::new(docs_button_label);
                            let docs_button_ref = ui.add_sized(vec2(60.0, 20.0), docs_button);
                            if docs_button_ref.clicked() {
                                // Show Documentation
                            }
                        });

                        ui.add_space(10.0);
                    });

                // Buttons
                TopBottomPanel::bottom("bottom_right")
                    .frame(Frame::default().fill(Color32::from_gray(20)))
                    .exact_height(60.0)
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
                        let button_ref = ui.add_sized(ui.available_size(), button);
                        if button_ref.clicked() {
                            if is_running {
                                self.interrupt_code();
                            } else {
                                self.run_code(ctx);
                            }
                        }
                    });

                // Text Area
                ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                    let text_field = TextEdit::multiline(&mut self.code)
                        .code_editor()
                        .font(FontId::monospace(16.0));
                    ui.add_sized(
                        vec2(ui.available_width() - 2.0, ui.available_height()),
                        text_field,
                    );
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
