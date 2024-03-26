use crate::gui::app::App;
use crate::gui::canvas::object::ObjectView;
use eframe::egui::text::LayoutJob;
use eframe::egui::*;

impl App {
    pub fn canvas_view(&mut self, ctx: &Context) {
        let size = ctx.input(|i| i.viewport().outer_rect).unwrap();
        let main_frame_width = size.width() - Self::EDITOR_WIDTH;
        let main_frame_height = size.height() - Self::CONSOLE_HEIGHT;

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
                        painter.rect_filled(rect, Rounding::same(0.0), canvas.bg_color);

                        let content_painter = ui.painter_at(rect);

                        // Pictures
                        for config in &canvas.pictures {
                            let Some(texture) = canvas.image_textures.get(&config.name) else {
                                continue;
                            };
                            content_painter.image(
                                texture.id(),
                                Rect::from_min_size(
                                    canvas.to_canvas_coordinates(config.pos),
                                    config.size,
                                ),
                                Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                                Color32::WHITE,
                            );
                        }

                        // Lines
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
                    .exact_height(Self::CONSOLE_HEIGHT)
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
    }
}
