use crate::gui::app::App;
use crate::gui::editor::files::FileHandle;
use eframe::egui::*;

impl App {
    pub fn code_editor_view(&mut self, ctx: &Context) {
        SidePanel::right("right")
            .frame(Frame::default().fill(Color32::from_gray(20)))
            .exact_width(Self::EDITOR_WIDTH)
            .resizable(false)
            .show_separator_line(false)
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
                                let docs_button =
                                    Button::new(docs_button_label).fill(Color32::from_gray(60));
                                let docs_button_response =
                                    ui.add_sized(vec2(60.0, 20.0), docs_button);
                                if docs_button_response.clicked() {
                                    // Heh
                                }

                                // Syntax Highlighting
                                let highlight_button_label = RichText::new(String::from("Hi-Lite"))
                                    .font(FontId::proportional(14.0))
                                    .color(Color32::from_gray(255));
                                let highlight_button = Button::new(highlight_button_label)
                                    .fill(Color32::from_gray(60));
                                let highlight_button_response =
                                    ui.add_sized(vec2(60.0, 20.0), highlight_button);
                                if highlight_button_response.clicked() {
                                    self.editor.should_highlight = !self.editor.should_highlight;
                                }

                                // Save File
                                let save_button_label = RichText::new(String::from("Save"))
                                    .font(FontId::proportional(14.0))
                                    .color(Color32::from_gray(255));
                                let save_button =
                                    Button::new(save_button_label).fill(Color32::from_gray(60));
                                let save_button_response =
                                    ui.add_sized(vec2(60.0, 20.0), save_button);
                                if save_button_response.clicked() {
                                    self.editor.save_current_file();
                                }

                                // Open File
                                let open_button_label = RichText::new(String::from("Open"))
                                    .font(FontId::proportional(14.0))
                                    .color(Color32::from_gray(255));
                                let open_button =
                                    Button::new(open_button_label).fill(Color32::from_gray(60));
                                let open_button_response =
                                    ui.add_sized(vec2(60.0, 20.0), open_button);
                                if open_button_response.clicked() {
                                    self.editor.open_file();
                                }

                                // New File
                                let new_button_label = RichText::new(String::from("New"))
                                    .font(FontId::proportional(14.0))
                                    .color(Color32::from_gray(255));
                                let new_button =
                                    Button::new(new_button_label).fill(Color32::from_gray(60));
                                let new_button_response =
                                    ui.add_sized(vec2(60.0, 20.0), new_button);
                                if new_button_response.clicked() {
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
                                        self.file_tab_view(ui, file, index, current_file_index);
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
                        let button = Button::new(button_label).fill(Color32::from_gray(60));
                        let button_response = ui.add_sized(ui.available_size(), button);
                        if button_response.clicked() {
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
                                ui.style_mut().visuals.extreme_bg_color = Color32::from_gray(0);
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
    }

    fn file_tab_view(
        &mut self,
        ui: &mut Ui,
        file: &FileHandle,
        index: usize,
        current_index: Option<usize>,
    ) {
        let mut frame = Frame::default()
            .stroke(Stroke::new(1.0, Color32::from_gray(50)))
            .begin(ui);
        {
            frame.content_ui.set_min_size(vec2(140.0, 25.0));
            frame.content_ui.set_max_size(vec2(140.0, 25.0));
            frame
                .content_ui
                .with_layout(Layout::top_down(Align::LEFT), |ui: &mut Ui| {
                    ui.add_space(10.0);
                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui: &mut Ui| {
                        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                        let mut inner_frame = Frame::default().begin(ui);
                        {
                            inner_frame.content_ui.with_layout(
                                Layout::left_to_right(Align::TOP),
                                |ui: &mut Ui| {
                                    ui.add_space(15.0);
                                    let mut file_text = RichText::new(file.name.clone())
                                        .font(FontId::proportional(12.0))
                                        .color(Color32::from_gray(255));
                                    if file.is_edited {
                                        file_text = file_text.italics();
                                    }
                                    let file_label = Label::new(file_text).truncate(true);
                                    ui.add(file_label);
                                },
                            );
                            inner_frame.content_ui.add_space(10.0);
                        }
                        let response = inner_frame.allocate_space(ui);
                        let interact = response.interact(Sense::click());
                        if interact.clicked() {
                            self.editor.select_file(index.clone());
                        }
                        inner_frame.paint(ui);

                        ui.with_layout(Layout::right_to_left(Align::TOP), |ui: &mut Ui| {
                            ui.add_space(15.0);
                            let close_text = RichText::new(String::from("x"))
                                .font(FontId::proportional(12.0))
                                .color(Color32::from_gray(255));
                            let close_label = Label::new(close_text);
                            let close_label_response = ui
                                .add(close_label)
                                .on_hover_cursor(CursorIcon::PointingHand);
                            if close_label_response.clicked() {
                                self.editor.close_file(index.clone());
                            }
                        });
                    });
                    ui.add_space(10.0);
                });
        }
        let response = frame.allocate_space(ui);
        let is_selected = index == current_index.unwrap_or(0);
        if is_selected {
            frame.frame.fill = Color32::from_gray(50);
        } else if response.hovered() {
            frame.frame.fill = Color32::from_gray(30);
        } else {
            frame.frame.fill = Color32::from_gray(20);
        }
        frame.paint(ui);
    }
}
