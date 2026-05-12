use std::time::{Duration, Instant};

use crate::utils::terminal::{blue, green, purple, red, yellow};

pub struct StepTimer {
    verbose: bool,
    label: String,
    started_at: Instant,
    last_step_at: Instant,
}

impl StepTimer {
    pub fn new(verbose: bool, label: impl Into<String>) -> Self {
        let now = Instant::now();

        Self {
            verbose,
            label: label.into(),
            started_at: now,
            last_step_at: now,
        }
    }

    pub fn step(&mut self, name: &str) {
        if !self.verbose {
            return;
        }

        let now = Instant::now();
        let duration = now.duration_since(self.last_step_at);
        self.last_step_at = now;

        println!(
            "{} {:<24} {:<24} {}",
            purple("[STEP]"),
            self.label,
            name,
            format_duration(duration)
        );
    }

    pub fn finish(self) {
        if !self.verbose {
            return;
        }

        println!(
            "{} {:<24} {:<24} {}",
            green("[TIME]"),
            self.label,
            "total",
            format_duration(self.started_at.elapsed())
        );
    }
}

fn format_duration(duration: Duration) -> String {
    let millis = duration.as_secs_f64() * 1000.0;

    if millis < 10.0 {
        green(&format!("{:.3}ms", millis))
    } else if millis < 100.0 {
        blue(&format!("{:.3}ms", millis))
    } else if millis < 500.0 {
        yellow(&format!("{:.3}ms", millis))
    } else {
        red(&format!("{:.3}ms", millis))
    }
}
