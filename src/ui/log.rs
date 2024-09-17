use std::{cell::OnceCell, fmt::Display};
// use colored::Colorize;

static mut ENABLED: bool = true;
pub fn toggle() {
    unsafe {
        ENABLED ^= true;
    }
}
fn enabled() -> bool {
    unsafe { ENABLED }
}

pub struct Logger {
    pub enabled: bool,
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        println!("[info] {}", $($arg)*);
    };
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
        let log = "info"; //.green();
        println!("{log} {text}")
    }
    pub fn err(&self, text: impl Display) {
        if !self.enabled {
            return;
        }
        let error = "error"; //.bright_red().bold();
        println!("{error} {text}")
    }
    pub fn note(&self, text: impl Display) {
        let note = "note"; //.bright_blue().bold();
        println!("{note} {text}");
    }
    pub fn warn(&self, text: impl Display) {
        let warning = "warn"; //.yellow().bold();
        println!("{warning} {text}");
    }
}
