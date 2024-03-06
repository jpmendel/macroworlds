use crate::interpreter::interpreter::Interpreter;
use crate::language::event::UiEvent;
use crate::view::canvas::Canvas;
use crate::view::turtle::Turtle;
use eframe::egui::*;
use std::collections::HashMap;
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
    pub event_receiver: Arc<Mutex<mpsc::Receiver<UiEvent>>>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (sender, receiver) = mpsc::channel::<UiEvent>();
        let interpreter = Interpreter::new(sender);
        let canvas_pos = pos2(50.0, 50.0);
        let canvas_size = vec2(500.0, 400.0);
        App {
            interpreter: Arc::from(Mutex::from(interpreter)),
            code: String::new(),
            canvas_pos,
            canvas_size,
            canvas: Arc::from(Mutex::from(Canvas::new(canvas_pos, canvas_size))),
            event_receiver: Arc::from(Mutex::from(receiver)),
        }
    }

    pub fn run_code(&mut self, ctx: &Context) {
        // Set up a background thread to listen to UI events coming over the channel.
        let canvas_mutex = self.canvas.clone();
        let receiver_mutex = self.event_receiver.clone();
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
                canvas.draw(&ctx, event);
            }
        });

        // Set up a background thread to run interpreter and send any UI updates.
        let interpreter_mutex = self.interpreter.clone();
        let code = self.code.clone();
        thread::spawn(move || {
            let mut interpreter = interpreter_mutex.lock().unwrap();
            let _ = interpreter.interpret(&code);
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        let size = ctx.input(|i| i.viewport().outer_rect).unwrap();
        SidePanel::left("left")
            .frame(Frame::none())
            .exact_width(size.width() * 0.6)
            .resizable(false)
            .show(ctx, |ui: &mut Ui| {
                ui.heading("MicroWorlds.rs");
                let painter = ui.painter();
                let rect = Rect::from_x_y_ranges(
                    Rangef::new(self.canvas_pos.x, self.canvas_pos.x + self.canvas_size.x),
                    Rangef::new(self.canvas_pos.y, self.canvas_pos.y + self.canvas_size.y),
                );
                painter.rect_filled(rect, Rounding::same(0.0), Color32::from_rgb(255, 255, 255));
                let line_painter = ui.painter_at(rect);
                let canvas = self.canvas.lock().unwrap();
                for line in &canvas.lines {
                    line_painter.line_segment(
                        [line.start.clone(), line.end.clone()],
                        Stroke::new(3.0, line.color),
                    );
                }
            });
        SidePanel::right("right")
            .frame(Frame::none())
            .exact_width(size.width() * 0.4)
            .resizable(false)
            .show(ctx, |ui: &mut Ui| {
                ui.heading("Editor");
                let text_field = TextEdit::multiline(&mut self.code)
                    .code_editor()
                    .desired_rows(28)
                    .desired_width(size.width() * 0.39)
                    .font(FontId::monospace(16.0));
                ui.add(text_field);

                ui.centered_and_justified(|ui: &mut Ui| {
                    let button = Button::new(RichText::new(String::from("Run Code")));
                    let button_res = ui.add(button);
                    if button_res.clicked() {
                        self.run_code(ctx);
                    }
                });
            });
    }
}
