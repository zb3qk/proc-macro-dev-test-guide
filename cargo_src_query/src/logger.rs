use log::{info, LevelFilter};
use std::io::Write;

#[cfg(test)]
pub fn setup_logger() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}:{}] {} - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            )
        })
        // .filter(Some("logger_example"), LevelFilter::Debug)
        .filter_level(LevelFilter::Debug)
        .target(env_logger::Target::Stdout)
        .init();
}