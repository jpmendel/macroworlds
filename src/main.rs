mod gui;
mod interpreter;
mod test;

use eframe::egui::vec2;
use gui::app::App;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size(vec2(1200.0, 700.0))
            .with_resizable(true),
        ..Default::default()
    };
    let result = eframe::run_native(
        "MacroWorlds",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
    match result {
        Ok(..) => (),
        Err(err) => println!("error: {}", err),
    }
}
