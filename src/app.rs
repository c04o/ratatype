use std::time::Instant;

pub struct App {
    // what to type
    pub target_text: String,

    // what was typed
    pub user_input: String,

    // init clock
    pub start_time: Option<Instant>,

    // exit flag
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            // default test text
            target_text: "the quick brown fox jumps over the lazy dog".to_string(),

            // empty input
            user_input: String::new(),

            // no timer yet
            start_time: None,

            // keep running
            should_quit: false,
        }
    }

    pub fn calculate_wpm(&self) -> f64 {
        match self.start_time {
            // zero if not started
            None => 0.0,
            Some(start) => {
                // total minutes
                let elapsed = start.elapsed().as_secs_f64() / 60.0;

                // avoid div by zero
                if elapsed <= 0.0 {
                    return 0.0;
                }

                // wpm formula
                (self.user_input.len() as f64 / 5.0) / elapsed
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
