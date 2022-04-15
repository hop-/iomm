use crate::config::{
    attribute::{Attribute, Require},
    value::{Value, ValueType},
};

use crate::app::server::Server;

mod log;
mod app;
mod net;
mod core;
mod config;
mod error;

const DEFAULT_PORT: i64 = 7654;
const DEFAULT_LVL: &str = "info";

fn get_config_var_map() -> Vec<Attribute> {
    let mut m = Vec::new();

    // TODO: add mappings
    // Config mappings
    m.push(Attribute::new(
        "log_lvl",
        Some("IOMM_LOG_LVL"),
        Require::Default(Value::String(DEFAULT_LVL.to_string())),
        ValueType::String
    ).unwrap());
    m.push(Attribute::new(
        "port",
        Some("IOMM_PORT"),
        Require::Default(Value::Int(DEFAULT_PORT)),
        ValueType::Int
    ).unwrap());
    // ---------------

    m
}

fn main() {
    // Start
    println!("Hi, Iomm is here.");
    println!("----------------");

    // Initializing config
    match config::init(&get_config_var_map()) {
        Ok(_) => (),
        Err(es) => {
            for e in es {
                println!("{}", e);
            };

            // Panic?
            panic!("failed config init");
        },
    };

    // Initializing logger
    log::init();

    // Create a server with given port
    let port = config::get("port").unwrap().as_int().unwrap() as u16;
    let server = Server::new(port);

    // Starting the server
    server.serve();

    // Good bye
    println!("----------------");
    println!("Bye.");
}
