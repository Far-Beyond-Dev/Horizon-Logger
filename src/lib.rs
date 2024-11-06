use chrono::Local;
use colored::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing::{Level, Subscriber};
use tracing_subscriber::FmtSubscriber;

/// Log levels with corresponding colors
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    DEBUG,   // Cyan
    INFO,    // Green
    WARN,    // Yellow
    ERROR,   // Red
    CRITICAL // Bright Red background
}

impl LogLevel {
    fn color(&self) -> ColoredString {
        match self {
            LogLevel::DEBUG => format!("{:^7}", "DEBUG").cyan(),
            LogLevel::INFO => format!("{:^7}", "INFO").green(),
            LogLevel::WARN => format!("{:^7}", "WARN").yellow(),
            LogLevel::ERROR => format!("{:^7}", "ERROR").red(),
            LogLevel::CRITICAL => format!("{:^7}", "CRIT").on_red().white(),
        }
    }
}

/// Log entry structure for storing log history
#[derive(Debug, Clone)]
struct LogEntry {
    timestamp: String,
    level: LogLevel,
    component: String,
    message: String,
}

/// Global log history
static LOG_HISTORY: Lazy<Mutex<Vec<LogEntry>>> = Lazy::new(|| Mutex::new(Vec::new()));

/// Initialize the logging system
pub fn init() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter("info")
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .init();
}

/// Main logging implementation
pub struct HorizonLogger;

impl HorizonLogger {
    /// Create new logger instance
    pub fn new() -> Self {
        HorizonLogger
    }

    /// Log a debug message
    pub fn debug(&self, component: &str, message: &str) {
        self.log(LogLevel::DEBUG, component, message);
    }

    /// Log an info message
    pub fn info(&self, component: &str, message: &str) {
        self.log(LogLevel::INFO, component, message);
    }

    /// Log a warning message
    pub fn warn(&self, component: &str, message: &str) {
        self.log(LogLevel::WARN, component, message);
    }

    /// Log an error message
    pub fn error(&self, component: &str, message: &str) {
        self.log(LogLevel::ERROR, component, message);
    }

    /// Log a critical message
    pub fn critical(&self, component: &str, message: &str) {
        self.log(LogLevel::CRITICAL, component, message);
    }

    /// Internal logging function
    fn log(&self, level: LogLevel, component: &str, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let thread_info = format!("[{:?}]", std::thread::current().id()).purple();
        
        // Format the log message with colors
        let formatted_component = format!("[{}]", component).blue();
        
        println!("{} {} {} {} {}", 
            timestamp.white(),
            level.color(),
            thread_info,
            formatted_component,
            message
        );

        // Store in history
        let entry = LogEntry {
            timestamp,
            level,
            component: component.to_string(),
            message: message.to_string(),
        };

        if let Ok(mut history) = LOG_HISTORY.lock() {
            history.push(entry);
            
            // Keep only last 1000 entries
            if history.len() > 1000 {
                history.remove(0);
            }
        }
    }

    /// Get log history
    pub fn get_history(&self) -> Vec<LogEntry> {
        LOG_HISTORY.lock()
            .map(|history| history.clone())
            .unwrap_or_default()
    }
}

// Convenience macros
#[macro_export]
macro_rules! log_debug {
    ($logger:expr, $component:expr, $($arg:tt)*) => {
        $logger.debug($component, &format!($($arg)*))
    }
}

#[macro_export]
macro_rules! log_info {
    ($logger:expr, $component:expr, $($arg:tt)*) => {
        $logger.info($component, &format!($($arg)*))
    }
}

#[macro_export]
macro_rules! log_warn {
    ($logger:expr, $component:expr, $($arg:tt)*) => {
        $logger.warn($component, &format!($($arg)*))
    }
}

#[macro_export]
macro_rules! log_error {
    ($logger:expr, $component:expr, $($arg:tt)*) => {
        $logger.error($component, &format!($($arg)*))
    }
}

#[macro_export]
macro_rules! log_critical {
    ($logger:expr, $component:expr, $($arg:tt)*) => {
        $logger.critical($component, &format!($($arg)*))
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        let logger = HorizonLogger::new();
        
        logger.debug("TEST", "This is a debug message");
        logger.info("TEST", "This is an info message");
        logger.warn("TEST", "This is a warning message");
        logger.error("TEST", "This is an error message");
        logger.critical("TEST", "This is a critical message");
        
        let history = logger.get_history();
        assert_eq!(history.len(), 5);
    }
}

// horizon_logger/src/examples.rs
pub fn example_usage() {
    let logger = HorizonLogger::new();
    
    // Basic logging
    logger.debug("SYSTEM", "Initializing server...");
    logger.info("NETWORK", "Player connected from 192.168.1.1");
    logger.warn("GAME", "Player attempted invalid move");
    logger.error("DATABASE", "Failed to save player state");
    logger.critical("SECURITY", "Detected potential security breach");
    
    // Using macros
    log_info!(logger, "PLAYER", "Player {} joined the game", "John");
    log_warn!(logger, "PHYSICS", "Collision detection took {}ms", 150);
    
    // Multiple components
    logger.info("GAME/COMBAT", "Player dealt 50 damage");
    logger.debug("NETWORK/WEBSOCKET", "Processing message batch");
}
