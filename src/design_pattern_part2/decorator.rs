use chrono::{Utc, DateTime};

trait Component {
    fn get_log_message(&self, msg: &str) -> String;
}

struct Logger;

impl Logger {
    fn new() -> Self { Self }
}

impl Component for Logger {
    fn get_log_message(&self, msg: &str) -> String {
        msg.to_string()
    }
}

struct TimestampDecorator<T: Component> {
    component: T
}

impl<T: Component> TimestampDecorator<T> {
    fn new(component: T) -> Self {
        Self { component }
    }
}

impl<T: Component> Component for TimestampDecorator<T> {
    fn get_log_message(&self, msg: &str) -> String {
        let local_date: DateTime<Utc> = Utc::now();
        format!("{} {}", local_date, self.component.get_log_message(msg))
    }
}

struct LogLevelDecorator<T: Component> {
    component: T,
    log_level: String,
}

impl<T: Component> LogLevelDecorator<T> {
    fn new(component: T, log_level: String) -> Self {
        Self {
            component,
            log_level,
        }
    }
}

impl<T: Component> Component for LogLevelDecorator<T> {
    fn get_log_message(&self, msg: &str) -> String {
        format!("{} {}", self.log_level, self.component.get_log_message(msg))
    }
}

pub struct DecoratorMain;

impl DecoratorMain {
    pub fn index() {
        let logger = Logger::new();
        println!("{}", logger.get_log_message("Design Pattern!"));
        let log_level_logger = LogLevelDecorator::new(logger, "INFO".to_string());
        println!("{}", log_level_logger.get_log_message("Design Pattern!"));
        let timestamp_logger = TimestampDecorator::new(log_level_logger);
        println!("{}", timestamp_logger.get_log_message("Design Pattern!"));
    }
}