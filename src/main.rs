use std::sync::{Arc, Mutex};
use std::{thread, time};

mod app;
mod stopwatch;

use stopwatch::Stopwatch;

fn main() -> eframe::Result<(), eframe::Error> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Stopwatch App",
        native_options,
        Box::new(|context| Box::new(app::App::new(context))),
    )?;
    Ok(())
}
