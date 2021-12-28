use crate::config::{
    attribute::{Attribute, Require}, 
    value::Value,
    error::Error,
};

use std::{
    collections::HashMap,
    env,
    sync::{
        atomic::{AtomicBool, Ordering},
        Once,
    },
};

use dotenv;

pub mod value;
pub mod attribute;
pub mod error;

static IS_INIT: AtomicBool = AtomicBool::new(false);

fn instance<'a>() -> &'a mut HashMap<String, Value> {
    static mut CONFIG: Option<HashMap<String, Value>> = None;
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            let map = HashMap::new();
            CONFIG.replace(map);
        });

        CONFIG.as_mut().unwrap()
    }
}

fn panic_if_init() {
    if IS_INIT.load(Ordering::Relaxed)
    {
        panic!("Config init called more then once");
    }
}

fn set_init() {
    IS_INIT.store(true, Ordering::Relaxed);
}

pub fn try_init_env(attr_map: &Vec<Attribute>) -> Result<(), Vec<Error>> {
    let mut errors = Vec::new();

    for attr in attr_map.iter() {
        if attr.var.is_none() {
            continue;
        }

        let var_name = attr.var.as_ref().unwrap();

        let value = {
            match env::var(var_name) {
                Ok(r) => r,
                Err(_) => {
                    if attr.required == Require::True {
                        errors.push(Error {message: format!("missing required variable '{}'", var_name)});
                    };

                    continue;
                },
            }
        };

        match Value::new_of(&value, &attr.value_type) {
            Ok(value) => { instance().insert(attr.name.clone(), value); },
            Err(e) => { errors.push(Error {message: e.message}); },
        };
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
}

fn set_defaults(attr_map: &Vec<Attribute>) {
    for attr in attr_map {
        match &attr.required {
            Require::Default(v) => {instance().insert(attr.name.clone(), v.clone());},
            _ => (),
        }
    }
}

pub fn init_env(attr_map: &Vec<Attribute>) -> Result<(), Vec<Error>> {
    panic_if_init();

    set_defaults(attr_map);

    let r = try_init_env(attr_map);

    set_init();

    r
}

fn try_init_dotenv(attr_map: &Vec<Attribute>) -> Result<(), Vec<Error>> {
    // Init dotenv to get vars for .env
    let mut errors = Vec::new();
    match dotenv::dotenv() {
        Ok(_) => (), // Skip .env
        Err(_) => errors.push(Error{message: "unable to find '.env' in current dir".to_string()}),
    };

    match try_init_env(attr_map) {
        Ok(_) => Ok(()),
        Err(mut es) => { errors.append(&mut es); Err(errors) },
    }
}

pub fn init_dotenv(attr_map: &Vec<Attribute>) -> Result<(), Vec<Error>> {
    panic_if_init();

    set_defaults(attr_map);

    let r = try_init_dotenv(attr_map);

    set_init();

    r
}

fn try_init_yaml(_attr_map: &Vec<Attribute>) -> Result<(), Vec<Error> > {
    // TODO: implement
    Ok(())
}

pub fn init_yaml(attr_map: &Vec<Attribute>) -> Result<(), Vec<Error>> {
    panic_if_init();

    set_defaults(attr_map);

    let r = try_init_yaml(attr_map);

    set_init();

    r
}

pub fn init(attr_map: &Vec<Attribute>) -> Result<(), Vec<Error>> {
    panic_if_init();

    set_defaults(attr_map);

    // Init with dotenv
    try_init_dotenv(attr_map).ok();
    // Init with yaml
    try_init_yaml(attr_map).ok();

    set_init();

    // Check required attributes
    check_required_attributs(attr_map)
}

fn check_required_attributs(var_map: &Vec<Attribute>) -> Result<(), Vec<Error>>{
    let mut errors = Vec::new();

    for attr in var_map {
        if attr.required != Require::True || get(&attr.name).is_some() {
            continue;
        }

        errors.push(Error {message: format!("required config attribute '{}' is missing", attr.name)});
    }

    if !errors.is_empty() {
        return Err(errors)
    }

    Ok(())
}

pub fn get<'a>(key: &str) -> Option<&'a Value> {
    instance().get(&key.to_string())
} 
