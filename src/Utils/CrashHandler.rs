use std::alloc::System;
use std::backtrace::Backtrace;
use std::panic;
use std::sync::mpsc;
use libc;
use chrono::Utc;
use libc::c_int;
use serde::{Deserialize, Serialize};
use winapi::um::consoleapi::SetConsoleCtrlHandler;

#[derive(Debug, Serialize, Deserialize)]
struct CrashReport {
    timestamp: String,
    file: Option<String>,
    line: Option<u32>,
    message: String,
    call_stack: Option<String>,
}

pub struct CrashHandler {
    logger: Logger,
}

struct Signals<'a>(&'a [c_int; 2]);

impl CrashHandler {
    pub fn new() -> Self {
        let handler = Self {
            logger: Logger::new(),
        };
     //   handler.setup_hooks();
        handler
    }

    pub fn setup_hooks(&self) {
        let logger = self.logger.clone();
        let termination_flag = Arc::new(AtomicBool::new(false));

        // Set up panic hook
        {
            let logger = logger.clone();
            let termination_flag = Arc::clone(&termination_flag);

            panic::set_hook(Box::new(move |panic_info| {
                let backtrace = Backtrace::capture();
                let (tx, rx) = mpsc::channel();

                // Extract panic information
                let payload = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                    s.clone()
                } else {
                    "Unknown panic payload".to_string()
                };

                let location = if let Some(location) = panic_info.location() {
                    format!("file: '{}', line: {}", location.file(), location.line())
                } else {
                    "Unknown location".to_string()
                };

                let logger_clone = logger.clone();
                let termination_flag_clone = Arc::clone(&termination_flag);
                thread::spawn(move || {
                    if let Err(e) = Self::handle_panic(payload, location, backtrace, &logger_clone, tx, &termination_flag_clone) {
                        logger_clone.log_error("CrashHandler", &format!("Failed to handle panic: {}", e));
                    }
                });

                if let Err(e) = rx.recv() {
                    logger.log_error("CrashHandler", &format!("Crash handling thread encountered an error: {}", e));
                }
            }));
        }
    }

    fn handle_panic(
        payload: String,
        location: String,
        backtrace: Backtrace,
        logger: &Logger,
        tx: mpsc::Sender<()>,
        termination_flag: &Arc<AtomicBool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        logger.log_error("CrashHandler", &format!("Panic occurred: {}", payload));
        logger.log_error("CrashHandler", &format!("Location: {}", location));
        logger.log_error("CrashHandler", &format!("Backtrace: {:?}", backtrace));

        if termination_flag.load(Ordering::SeqCst) {
            logger.log("CrashHandler", "Termination signal received during panic handling and has been DISCARDED.");
        }


        tx.send(()).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(())
    }

    fn create_crash_report(panic_info: &panic::PanicInfo, backtrace: Backtrace) -> CrashReport {
        CrashReport {
            timestamp: Utc::now().to_rfc3339(),
            file: panic_info.location().map(|l| l.file().to_string()),
            line: panic_info.location().map(|l| l.line()),
            message: panic_info.payload().downcast_ref::<String>()
                .map(|s| s.clone())
                .unwrap_or_else(|| "Unknown panic".to_string()),
            call_stack: Some(Self::capture_call_stack(backtrace)),
        }
    }

    fn capture_call_stack(backtrace: Backtrace) -> String {
        format!("{:?}", backtrace)
    }


    fn log_crash_report(crash_report: &CrashReport, logger: &Logger) -> Result<(), Box<dyn std::error::Error>> {
        logger.log_error("CrashHandler", &format!("Crash Report: {:?}", crash_report));
        Ok(())
    }

    fn write_crash_report_to_file(crash_report: &CrashReport, logger: &Logger) -> Result<(), Box<dyn std::error::Error>> {
        let log_message = serde_json::to_string(crash_report)?;
        logger.log_error("CrashHandler", &log_message);
        Ok(())
    }

    fn pause_execution(logger: &Logger) {
        logger.log("CrashHandler", "Application paused. Press Enter to continue...");
        let mut input = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            logger.log_error("CrashHandler", &format!("Failed to read line: {}", e));
        }
    }
}
