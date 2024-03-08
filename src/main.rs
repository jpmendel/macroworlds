mod interpreter;
mod language;
mod state;
mod view;

use eframe::egui::vec2;
use view::app::App;

static DEBUG: bool = false;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size(vec2(1000.0, 600.0))
            .with_resizable(false),
        ..Default::default()
    };
    let result = eframe::run_native(
        "MicroWorlds.rs",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
    match result {
        Ok(..) => (),
        Err(err) => println!("error: {}", err),
    }
}
