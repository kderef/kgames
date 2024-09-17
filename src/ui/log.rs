use colored::Colorize;
use std::fmt::Display;

pub struct Logger {
    pub enabled: bool,
}

impl Logger {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    pub fn log(&self, text: impl Display) {
        if !self.enabled {
            return;
        }

        // Log the information
        let log = "info".green();
        println!("{log} {text}")
    }
    pub fn err(&self, text: impl Display) {
        if !self.enabled {
            return;
        }
        let error = "error".bright_red().bold();
        println!("{error} {text}")
    }
    pub fn note(&self, text: impl Display) {
        let note = "note".bright_blue().bold();
        println!("{note} {text}");
    }
    pub fn warn(&self, text: impl Display) {
        let warning = "warn".yellow().bold();
        println!("{warning} {text}");
    }
}
