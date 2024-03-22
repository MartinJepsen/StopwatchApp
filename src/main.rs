
// eframe is a cross-platform framework that supports native and web apps
use eframe::egui;
use std::{collections::HashMap, time};
use std::rc::Rc;
use std::cell::RefCell;

struct App {
    name: String,
    stopwatch_manager: StopWatchManager,
    should_repaint: bool
}



impl Default for App {
    fn default() -> Self {
        Self {
            name: "Application".into(),
            stopwatch_manager: StopWatchManager::new(),
            should_repaint: false
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Stopwatch application");
            ui.vertical(|ui| {
                draw_stopwatches(ui, &mut self.stopwatch_manager);
                if ui.button("Add stopwatch").clicked() {
                    add_timer(ui, &mut self.stopwatch_manager, &ctx)
                }
            });
            // if add_button.clicked() {
            //     ui.label("I was clicked");
            //     add_timer(ui, &mut self.stopwatch_manager, &ctx);
            // }
            // ui.vertical_centered(
            //     |mut ui| {
            //         if ui.button("Add stopwatch").clicked() {
            //             add_timer(&mut ui, &mut self.stopwatch_manager, &ctx);
            //             self.should_repaint = true;
            //         } else {
            //             add_timer(&mut ui, &mut self.stopwatch_manager, &ctx)
            //         }
            //     }
            // )

        });
    }
}

struct Stopwatch {
    is_running: bool,
    start_time: Option<time::Instant>,
    elapsed_time: time::Duration,
}

impl Default for Stopwatch {
    fn default() -> Self {
        Stopwatch {is_running: false, start_time: None, elapsed_time: time::Duration::default()}
    }
}

impl Stopwatch {
    fn start_or_stop(&mut self) {
        if self.is_running {
            self.is_running = false;
            self.elapsed_time += self.start_time.unwrap().elapsed();
        } else {
            self.is_running = true;
            self.start_time = Some(time::Instant::now());
        }
    }

    fn reset(&mut self) {
        self.is_running = false;
        self.start_time = None;
        self.elapsed_time = time::Duration::default();
    }

    fn display(&self) -> String {
        let total_time = if self.is_running {
            self.elapsed_time + self.start_time.unwrap().elapsed()
        } else {
            self.elapsed_time
        };

        format!("{:02}:{:02}:{:02}", total_time.as_secs() / 60, total_time.as_secs(), total_time.subsec_millis() / 10)
    }
}


struct StopWatchManager {
    inner: HashMap<u8, Rc<RefCell<Stopwatch>>>
}

impl StopWatchManager {
    fn new() -> Self {
        Self {
            inner: HashMap::new()
        }
    }

    fn add(&mut self, id: u8) ->  Rc<RefCell<Stopwatch>> {
        let stopwatch = Rc::new(RefCell::new(Stopwatch::default()));
        self.inner.insert(id, Rc::clone(&stopwatch));
        stopwatch
    }

    fn update_stopwatch(&mut self, id: u8) {
        if let Some(stopwatch_ref) = self.inner.get_mut(&id) {
            let mut stopwatch = stopwatch_ref.borrow_mut();
            if stopwatch.is_running {
                let start_time = stopwatch.start_time.unwrap().elapsed();
                stopwatch.elapsed_time += start_time;
                stopwatch.start_time = Some(time::Instant::now())
            }
        };
    }

    fn start_or_stop_stopwatch(&mut self, id: &u8)  {
        if let Some(stopwatch_ref) = self.inner.get_mut(id) {
            let mut stopwatch = stopwatch_ref.borrow_mut();
            stopwatch.start_or_stop();
        }
    }
}


fn draw_stopwatches(ui: &mut egui::Ui, manager: &mut StopWatchManager) {
    let mut sorted_stopwatches: Vec<_> = manager.inner.clone().into_iter().collect();
    sorted_stopwatches.sort_by_key(|&(id, _)| id);
    for (id, stopwatch_ref) in sorted_stopwatches {
        ui.horizontal(|ui| {
            ui.label(format!("Stopwatch {:02}", id));
            ui.label(stopwatch_ref.borrow().display());
    
            if ui.button("Start").clicked() {
                println!("Click")
            };
        });
    }
}


fn add_timer(ui: &mut egui::Ui, manager: &mut StopWatchManager, context: &egui::Context) {
    
    let id: u8 = (manager.inner.len() as u8) + 1;
    let _ = manager.add(id);
    println!("Added stopwatch with id {id}");


    
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        // "ViewPort" is the entire native OS window that the GUI resides in
        viewport: egui::ViewportBuilder::default().with_inner_size([620.0, 620.0]),
        ..Default::default()
    };
    let context = egui::Context::default();
    let app = App::default();

    eframe::run_native(
        "Stopwatch app",
        options,
        Box::new(|cc| Box::new(app))
    ).unwrap();
    Ok(())
}
