pub fn init_logging() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap_or(
        log4rs::init_file("/app/configuration/log4rs.yml", Default::default()).unwrap_or(()),
    );
}
