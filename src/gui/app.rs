use crate::gui::canvas::CanvasView;
use crate::gui::editor::Editor;
use crate::gui::object::ObjectView;
use crate::interpreter::event::InputEvent;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::state::object::Point;
use crate::interpreter::state::state::State;
use eframe::egui::text::LayoutJob;
use eframe::egui::*;
use std::collections::HashSet;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct App {
    interpreter: Arc<Mutex<Interpreter>>,
    canvas: Arc<Mutex<CanvasView>>,
    editor: Editor,
    input_sender: mpsc::Sender<InputEvent>,
    current_keys: HashSet<String>,
    is_running: Arc<Mutex<bool>>,
}

impl App {
    const EDITOR_WIDTH: f32 = 480.0;
    const CONSOLE_HEIGHT: f32 = 160.0;

    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (input_sender, input_receiver) = mpsc::channel::<InputEvent>();
        let mut interpreter = Interpreter::new();
        interpreter.bind_input_receiver(input_receiver);
        let canvas_size = vec2(State::DEFAULT_CANVAS_WIDTH, State::DEFAULT_CANVAS_HEIGHT);
        App {
            interpreter: Arc::from(Mutex::from(interpreter)),
            canvas: Arc::from(Mutex::from(CanvasView::new(canvas_size))),
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
        let new_canvas = CanvasView::new(vec2(
            State::DEFAULT_CANVAS_WIDTH.clone(),
            State::DEFAULT_CANVAS_HEIGHT.clone(),
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
                // Canvas
                CentralPanel::default()
                    .frame(Frame::default().fill(Color32::from_gray(20)))
                    .show_inside(ui, |ui: &mut Ui| {
                        let mut canvas = self.canvas.lock().unwrap();

                        // Blank Canvas
                        let painter = ui.painter();
                        let canvas_pos = pos2(
                            main_frame_width / 2.0 - canvas.size.x / 2.0,
                            main_frame_height / 2.0 - canvas.size.y / 2.0 + 6.0,
                        );
                        canvas.pos = canvas_pos;
                        let rect = Rect::from_x_y_ranges(
                            Rangef::new(canvas_pos.x, canvas_pos.x + canvas.size.x),
                            Rangef::new(canvas_pos.y, canvas_pos.y + canvas.size.y),
                        );
                        if let Some(texture) = &canvas.bg_picture {
                            painter.image(
                                texture.id(),
                                rect,
                                Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                                Color32::WHITE,
                            );
                        } else {
                            painter.rect_filled(rect, Rounding::same(0.0), canvas.bg_color);
                        }

                        // Lines
                        let content_painter = ui.painter_at(rect);
                        for config in &canvas.drawn_paths {
                            content_painter.add(canvas.path_for_config(config));
                        }
                        for (_, config) in &canvas.current_turtle_paths {
                            content_painter.add(canvas.path_for_config(config));
                        }

                        // Turtles and Text
                        for (_, obj) in &canvas.objects {
                            match obj {
                                ObjectView::Turtle(turtle) => {
                                    if turtle.is_visible {
                                        if let Some(shape) = canvas.shape_for_turtle(turtle) {
                                            content_painter.add(shape);
                                        }
                                    }
                                }
                                ObjectView::Text(text) => {
                                    if text.is_visible {
                                        let mut format = TextFormat::simple(
                                            FontId::proportional(text.font_size),
                                            text.color,
                                        );
                                        format.italics = text.is_italic();
                                        format.underline = if text.is_underlined() {
                                            Stroke::new(text.font_size / 20.0, text.color)
                                        } else {
                                            Stroke::NONE
                                        };
                                        let mut job = LayoutJob::single_section(
                                            text.text.to_string(),
                                            format,
                                        );
                                        job.halign = Align::Center;
                                        let galley = content_painter.layout_job(job);
                                        content_painter.galley(
                                            canvas.to_canvas_coordinates(text.pos),
                                            galley,
                                            text.color,
                                        );
                                    }
                                }
                            }
                        }

                        // Handle Announcements
                        if canvas.is_window_open.clone() {
                            let announcement = canvas.announce_text.clone();
                            Window::new("Announcement")
                                .open(&mut canvas.is_window_open)
                                .resizable(false)
                                .collapsible(false)
                                .movable(true)
                                .anchor(
                                    Align2::LEFT_TOP,
                                    vec2(main_frame_width / 2.0, main_frame_height / 2.0),
                                )
                                .show(ctx, |ui: &mut Ui| {
                                    ui.add_space(10.0);
                                    ui.with_layout(
                                        Layout::left_to_right(Align::TOP),
                                        |ui: &mut Ui| {
                                            ui.add_space(10.0);
                                            ui.label(
                                                RichText::new(announcement)
                                                    .font(FontId::proportional(18.0)),
                                            );
                                            ui.add_space(10.0);
                                        },
                                    );
                                    ui.add_space(10.0);
                                });
                        }
                    });

                // Header Bar
                TopBottomPanel::top("top_left")
                    .frame(Frame::default().fill(Color32::from_gray(20)))
                    .resizable(false)
                    .show_inside(ui, |ui: &mut Ui| {
                        ui.add_space(10.0);
                        ui.with_layout(Layout::left_to_right(Align::TOP), |ui: &mut Ui| {
                            ui.with_layout(Layout::left_to_right(Align::TOP), |ui: &mut Ui| {
                                // Title
                                ui.add_space(10.0);
                                let title = RichText::new(String::from("MacroWorlds"))
                                    .font(FontId::proportional(18.0))
                                    .color(Color32::from_gray(255));
                                let title_label = Label::new(title);
                                ui.add(title_label);
                            });

                            ui.with_layout(Layout::right_to_left(Align::TOP), |ui: &mut Ui| {
                                // Reset State and Variables
                                ui.add_space(10.0);
                                let reset_button_label = RichText::new(String::from("Reset"))
                                    .font(FontId::proportional(14.0))
                                    .color(Color32::from_gray(255));
                                let reset_button = Button::new(reset_button_label);
                                let reset_button_ref = ui.add_sized(vec2(60.0, 20.0), reset_button);
                                if reset_button_ref.clicked() {
                                    self.reset_state();
                                }
                            });
                        });
                        ui.add_space(10.0);
                    });

                // Output Console
                TopBottomPanel::bottom("bottom_left")
                    .frame(Frame::default().fill(Color32::from_gray(40)))
                    .exact_height(App::CONSOLE_HEIGHT)
                    .resizable(false)
                    .show_inside(ui, |ui: &mut Ui| {
                        ui.add_space(6.0);

                        ui.with_layout(Layout::left_to_right(Align::TOP), |ui: &mut Ui| {
                            let canvas = self.canvas.lock().unwrap();
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
                let current_file_index = self.editor.current_file_index.clone();

                // Toolbar
                TopBottomPanel::top("top_right")
                    .frame(Frame::default().fill(Color32::from_gray(20)))
                    .resizable(false)
                    .show_inside(ui, |ui: &mut Ui| {
                        ui.add_space(10.0);

                        ui.with_layout(Layout::left_to_right(Align::TOP), |ui: &mut Ui| {
                            ui.with_layout(Layout::left_to_right(Align::TOP), |ui: &mut Ui| {
                                // Title
                                ui.add_space(10.0);
                                let title = RichText::new(String::from("Code"))
                                    .font(FontId::proportional(18.0))
                                    .color(Color32::from_gray(255));
                                let title_label = Label::new(title);
                                ui.add(title_label);
                            });

                            // Buttons
                            ui.with_layout(Layout::right_to_left(Align::TOP), |ui: &mut Ui| {
                                ui.add_space(10.0);

                                // Show Documentation
                                let docs_button_label = RichText::new(String::from("Docs"))
                                    .font(FontId::proportional(14.0))
                                    .color(Color32::from_gray(255));
                                let docs_button = Button::new(docs_button_label);
                                let docs_button_ref = ui.add_sized(vec2(60.0, 20.0), docs_button);
                                if docs_button_ref.clicked() {
                                    // Heh
                                }

                                // Syntax Highlighting
                                let highlight_button_label = RichText::new(String::from("Hi-Lite"))
                                    .font(FontId::proportional(14.0))
                                    .color(Color32::from_gray(255));
                                let highlight_button = Button::new(highlight_button_label);
                                let highlight_button_ref =
                                    ui.add_sized(vec2(60.0, 20.0), highlight_button);
                                if highlight_button_ref.clicked() {
                                    self.editor.should_highlight = !self.editor.should_highlight;
                                }

                                // Save File
                                let save_button_label = RichText::new(String::from("Save"))
                                    .font(FontId::proportional(14.0))
                                    .color(Color32::from_gray(255));
                                let save_button = Button::new(save_button_label);
                                let save_button_ref = ui.add_sized(vec2(60.0, 20.0), save_button);
                                if save_button_ref.clicked() {
                                    self.editor.save_current_file();
                                }

                                // Open File
                                let open_button_label = RichText::new(String::from("Open"))
                                    .font(FontId::proportional(14.0))
                                    .color(Color32::from_gray(255));
                                let open_button = Button::new(open_button_label);
                                let open_button_ref = ui.add_sized(vec2(60.0, 20.0), open_button);
                                if open_button_ref.clicked() {
                                    self.editor.open_file();
                                }

                                // New File
                                let new_button_label = RichText::new(String::from("New"))
                                    .font(FontId::proportional(14.0))
                                    .color(Color32::from_gray(255));
                                let new_button = Button::new(new_button_label);
                                let new_button_ref = ui.add_sized(vec2(60.0, 20.0), new_button);
                                if new_button_ref.clicked() {
                                    self.editor.new_file();
                                }
                            });
                        });

                        // File Tabs
                        if !self.editor.open_files.is_empty() {
                            ui.add_space(10.0);
                            ScrollArea::horizontal().show(ui, |ui: &mut Ui| {
                                ui.with_layout(Layout::left_to_right(Align::TOP), |ui: &mut Ui| {
                                    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                                    let files = self.editor.open_files.clone();
                                    for (index, file) in files.iter().enumerate() {
                                        let bg_color = if index == current_file_index.unwrap_or(0) {
                                            Color32::from_gray(50)
                                        } else {
                                            Color32::from_gray(20)
                                        };
                                        let frame = Frame::default().fill(bg_color);
                                        frame.show(ui, |ui: &mut Ui| {
                                            ui.set_max_size(vec2(120.0, 30.0));
                                            ui.with_layout(
                                                Layout::top_down(Align::LEFT),
                                                |ui: &mut Ui| {
                                                    ui.add_space(7.5);
                                                    ui.with_layout(
                                                        Layout::left_to_right(Align::TOP),
                                                        |ui: &mut Ui| {
                                                            ui.spacing_mut().item_spacing =
                                                                vec2(0.0, 0.0);
                                                            ui.add_space(15.0);
                                                            let mut file_text =
                                                                RichText::new(file.name.clone())
                                                                    .font(FontId::proportional(
                                                                        12.0,
                                                                    ))
                                                                    .color(Color32::from_gray(255));
                                                            if file.is_edited {
                                                                file_text = file_text.italics();
                                                            }
                                                            let file_text_label =
                                                                Label::new(file_text)
                                                                    .truncate(true);
                                                            let file_text_label_ref = ui
                                                                .add(file_text_label)
                                                                .on_hover_and_drag_cursor(
                                                                    CursorIcon::Default,
                                                                );
                                                            if file_text_label_ref.clicked() {
                                                                self.editor
                                                                    .select_file(index.clone());
                                                            }
                                                            ui.add_space(10.0);
                                                            let close_text =
                                                                RichText::new(String::from("x"))
                                                                    .font(FontId::proportional(
                                                                        12.0,
                                                                    ))
                                                                    .color(Color32::from_gray(255));
                                                            let close_text_label =
                                                                Label::new(close_text)
                                                                    .truncate(true);
                                                            let close_text_label_ref = ui
                                                                .add(close_text_label)
                                                                .on_hover_and_drag_cursor(
                                                                    CursorIcon::PointingHand,
                                                                );
                                                            if close_text_label_ref.clicked() {
                                                                self.editor
                                                                    .close_file(index.clone());
                                                            }
                                                            ui.add_space(15.0);
                                                        },
                                                    );
                                                    ui.add_space(7.5);
                                                },
                                            );
                                        });
                                    }
                                });
                            });
                        } else {
                            ui.add_space(40.0);
                        }
                    });

                // Run/Stop Code Button
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
                CentralPanel::default()
                    .frame(Frame::default().fill(Color32::from_gray(20)))
                    .show_inside(ui, |ui: &mut Ui| {
                        if let Some(index) = current_file_index {
                            ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                                let font = self.editor.font.clone();
                                let size = vec2(ui.available_width() - 2.0, ui.available_height());
                                if self.editor.should_highlight {
                                    let highlighter = self.editor.highlighter.clone();
                                    let Some(file) = self.editor.get_file_mut(index) else {
                                        return;
                                    };
                                    let mut layouter = |ui: &Ui, text: &str, wrap_width: f32| {
                                        let job = highlighter.highlight(ui.ctx(), text, wrap_width);
                                        ui.fonts(|font| font.layout_job(job))
                                    };
                                    let text_field = TextEdit::multiline(file)
                                        .code_editor()
                                        .font(font)
                                        .layouter(&mut layouter);
                                    ui.add_sized(size, text_field);
                                } else {
                                    let Some(file) = self.editor.get_file_mut(index) else {
                                        return;
                                    };
                                    let text_field =
                                        TextEdit::multiline(file).code_editor().font(font);
                                    ui.add_sized(size, text_field);
                                }
                            });
                        }
                    });
            });

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
