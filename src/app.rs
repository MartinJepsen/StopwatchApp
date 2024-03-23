use crate::Stopwatch;
use std::{
    collections::{HashMap, HashSet},
    time,
};
use indexmap::IndexMap;

/// Application struct.
pub struct App {
    /// Mapping between IDs and `Stopwatch` instances.
    stopwatches: IndexMap<u8, Stopwatch>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            stopwatches: IndexMap::<u8, Stopwatch>::new(),
        }
    }
}

impl App {
    pub fn new(_context: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        // We use a `HashMap` for keeping track of which stopwatches should be removed at the end of this method
        let mut stopwatches_to_remove = HashSet::<u8>::new();
        // We create a panel with a grid.
        egui::CentralPanel::default().show(context, |ui| {
            egui::Grid::new("Stopwatches").striped(true).show(ui, |ui| {
                // If there are no stopwatches, we add a dummy label.
                if self.stopwatches.is_empty() {
                    ui.label("No stopwatches to display.");
                    ui.end_row();
                } else {
                    for (id, sw) in self.stopwatches.iter_mut() {
                        ui.label(format!("{:02}", id));
                        // Start/continue button
                        let start_continue_button =
                            ui.add(egui::widgets::Button::new("Start/continue").frame(false));
                        if start_continue_button.clicked() {
                            sw.start()
                        }
                        // Pause button
                        let pause_button = ui.add(egui::widgets::Button::new("Pause").frame(false));
                        if pause_button.clicked() {
                            sw.pause()
                        }
                        
                        // Label for displaying the elapsed time
                        ui.add(egui::Label::new(match sw.total_elapsed_time() {
                            Some(time) => format!("{:.4}", time.as_secs_f32()),
                            None => "Not started".to_owned(),
                        }));
                        // Delete button
                        // If clicked, it pushes the current `Stopwatch` ID to the set of IDs we want to delete
                        let delete_button = ui.add(egui::widgets::Button::new("X").frame(false));
                        if delete_button.clicked() {
                            stopwatches_to_remove.insert(id.clone());
                        }
                        ui.end_row();
                    }
                }

                // Button below the grid of stopwatches for adding new ones
                if ui.button("Add stopwatch").clicked() {
                    let id = match self.stopwatches.keys().max() {
                        Some(id) => id + 1,
                        None => 0,
                    };
                    self.stopwatches.insert(id, Stopwatch::new(id));
                }
            });
            // We explicitly request a repaint 60 times per second to make the stopwatches update continuously.
            context.request_repaint_after(time::Duration::from_secs_f32(1.0 / 60.0));
        });

        // Delete the stopwatches whose delete button has been clicked
        for id in stopwatches_to_remove {
            self.stopwatches.shift_remove(&id);
        }
    }
}
