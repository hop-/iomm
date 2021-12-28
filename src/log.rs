use crate::config;

use std::io::Write;

use env_logger::Builder;

pub fn init() {
    let mut log_builder = Builder::new();

    let lvl = config::get("log_lvl").unwrap().as_string().unwrap().as_str();

    log_builder.parse_filters(lvl);

    log_builder.format(|buf, record| {
        writeln!(buf, "{}: {}", record.level(), record.args())
    });

    log_builder.init();
}
