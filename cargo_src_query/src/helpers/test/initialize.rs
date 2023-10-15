
#[cfg(test)]
pub mod test {
    use std::sync::Once;
    use crate::logger::setup_logger;

    static START: Once = Once::new();
    pub fn initialize() {
        START.call_once(|| {
            setup_logger();
            color_eyre::install().expect("TODO: panic message");
        });
    }
}