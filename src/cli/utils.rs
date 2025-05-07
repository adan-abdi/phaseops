use crate::logger;

pub fn log_application(_company: &str, _role: &str) {
    logger::info("Utils Module (log_application): supposed to log an application to applications.csv");
}
