use std::sync::{Arc, Mutex};
use std::time;

#[derive(Debug, Clone)]
pub struct Stopwatch {
    pub id: u8,
    pub start_time: Option<time::Instant>,
    pub elapsed_time: Option<time::Duration>,
    pub is_running: Arc<Mutex<bool>>,
}

impl PartialEq for Stopwatch {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Stopwatch {
    pub fn new(id: u8) -> Self {
        Self {
            id,
            start_time: None,
            elapsed_time: None,
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    /// Start or continue the stopwatch
    pub fn start(&mut self) {
        let mut is_running = self.is_running.lock().unwrap();
        if !*is_running {
            self.start_time = Some(time::Instant::now());
            *is_running = true
        } else {
            println!("Stopwatch is already running")
        }
    }

    /// Pause the stopwatch and store the elapsed time.
    pub fn pause(&mut self) {
        match self.elapsed_time {
            // If `self.elapsed_time` already contains a value, we add the current elapsed time to it
            Some(elapsed_time) => {
                self.elapsed_time = Some(
                    elapsed_time
                        // `self.start_time` may be `None` if `pause` is called twice in a row.
                        // In that case, we put in a dummy duration
                        + match self.start_time {
                            Some(start_time) => start_time.elapsed(),
                            None => time::Duration::new(0, 0),
                        },
                )
            }
            // If no previous times have been stored yet
            None => {
                self.elapsed_time = match self.start_time {
                    // If timer wasn't started yet, pause does nothing to `self.elapsed_time`
                    None => None,
                    // If timer was started, store the elapsed time
                    Some(start_time) => Some(start_time.elapsed()),
                };
            }
        }
        self.stop();
    }

    /// Set the start time to `None`.
    pub fn stop(&mut self) {
        *self.is_running.lock().unwrap() = false;
        self.start_time = None;
    }

    /// Get the total elapsed time
    pub fn total_elapsed_time(&self) -> Option<time::Duration> {
        match self.start_time {
            // If there is a start time
            Some(start_time) => match self.elapsed_time {
                // Return the previously elapsed times + the time elapsed since the last start (if any)
                Some(elapsed_time) => Some(elapsed_time + start_time.elapsed()),
                None => Some(start_time.elapsed()),
            },
            // If there is not a start time
            None => match self.elapsed_time {
                // Return the elapsed time if there is one, else return `None`
                Some(elapsed_time) => Some(elapsed_time),
                None => None,
            },
        }
    }
}
