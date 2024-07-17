extern crate chrono;

use std::fmt;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::TimeZone;

#[derive(Debug)]
enum LogLevel {
    INFO,
    WARN,
    ERROR,
    FATAL,
    DEBUG,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::INFO => write!(f, "INFO"),
            LogLevel::WARN => write!(f, "WARN"),
            LogLevel::ERROR => write!(f, "ERROR"),
            LogLevel::FATAL => write!(f, "FATAL"),
            LogLevel::DEBUG => write!(f, "DEBUG"),
        }
    }
}

#[derive(Clone)]
pub struct Logger {
    splash: [&'static str; 12],
    colors: [u8; 5],
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            splash: [
                " __________________      _____  ___________ ___________ _______    ________.___ _______ ___________    ",
                "\\_   _____/\\      \\    /  _  \\ \\_   _____/ \\_   _____/ \\      \\  /  _____/|   |\\      \\ \\_   _____/    ",
                " |    __)  /   |   \\  /  /_\\  \\ |    __)    |    __)_  /   |   \\/   \\  ___|   |/   |   \\ |    __)_     ",
                " |     \\  /    |    \\/    |    \\|     \\     |        \\/    |    \\    \\_\\  \\   /    |    \\|        \\    ",
                " \\___  /  \\____|__  /\\____|__  /\\___  /    /_______  /\\____|__  /\\______  /___\\____|__  /_______  /    ",
                "     \\/           \\/         \\/     \\/             \\/         \\/        \\/            \\/        \\/     ",
                "                              __________ ____ ___  ____________________                            ",
                "                              \\______   \\    |   \\/   _____/\\__    ___/                            ",
                "      ______   ______   ______ |       _/    |   /\\_____  \\   |    |  ______   ______   ______     ",
                "     /_____/  /_____/  /_____/ |    |   \\    |  / /        \\  |    | /_____/  /_____/  /_____/     ",
                "                               |____|_  /______/ /_______  /  |____|                               ",
                "                                      \\/                 \\/                                        ",
            ],
            colors: [15, 11, 14, 12, 8],
        }
    }

    fn log_custom(&self, log_level: LogLevel, module: &str, message: &str, to_files: bool) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let formatted_timestamp = chrono::Utc.timestamp_opt(timestamp as i64, 0).unwrap().format("%H:%M:%S");

        print!(
            "{} {:?} ({}): {}\n",
            formatted_timestamp,
            log_level,
            module,
            message
        );

        if to_files {
            if let Ok(mut writer) = OpenOptions::new()
                .create(true)
                .append(true)
                .open("engine.log")
            {
                let mut buffered_writer = BufWriter::new(&mut writer);
                writeln!(
                    buffered_writer,
                    "{} {:?} ({}): {}",
                    formatted_timestamp, log_level, module, message
                )
                    .unwrap();
            }
        }
    }

    pub fn log(&self, module: &str, message: &str) {
        self.log_custom(LogLevel::INFO, module, message, true);
    }

    pub fn log_error(&self, module: &str, message: &str) {
        self.log_custom(LogLevel::ERROR, module, message, true);
    }

    pub fn log_warn(&self, module: &str, message: &str) {
        self.log_custom(LogLevel::WARN, module, message, true);
    }

    pub fn draw_splash(&self) {
        if let Ok(mut writer) = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("engine.log")
        {
            for line in self.splash.iter() {
                println!("{}", line);
                writeln!(writer, "{}", line).unwrap();
            }
        }
    }
}
