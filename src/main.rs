mod app;
mod chess;
mod game_mode;
mod stockfish;

use eframe::{egui, NativeOptions};

fn main() -> Result<(), eframe::Error> {
    // Set up logging if needed
    // env_logger::init();

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Chess Game",
        options,
        Box::new(|cc| Box::new(app::ChessApp::new(cc))),
    )
}
