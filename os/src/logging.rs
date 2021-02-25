#![allow(unused_macros)]

use log::{self, Level, LevelFilter, Log, Metadata, Record};

const DEFAULT_LOG_LEVEL: Level = Level::Info;

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let color = color_of_level(record.level());
        colored_println!(color, "[{:>5}] {}", record.level(), record.args());
    }

    fn flush(&self) {}
}

fn color_of_level(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // Bright Yellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // Bright Black
    }
}

#[allow(dead_code)]
pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        _ => DEFAULT_LOG_LEVEL.to_level_filter(),
    });
}
