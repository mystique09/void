use std::env;

pub fn init_default_logger() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();
}