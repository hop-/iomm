use crate::config::error::Error;

use std::{
    collections::HashMap,
    fmt,
};

#[derive(PartialEq)]
pub enum ValueType {
    None,
    Int,
    Float,
    String,
    Bool,
    Vec,
    Map,
}

#[derive(PartialEq, Clone)]
pub enum Value {
    None,
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Vec(Vec<Value>),
    Map(HashMap<String, Value>)
}

impl Value {
    // As there isn't static maps yet
    pub fn new_of(value: &String, value_type: &ValueType) -> Result<Value, Error> {
        match value_type {
            ValueType::Bool => Value::new_bool(value),
            ValueType::Int => Value::new_int(value),
            ValueType::Float => Value::new_float(value),
            ValueType::String => Value::new_string(value),
            ValueType::Vec => Value::new_vec(value),
            ValueType::Map => Value::new_map(value),
            _ => Ok(Value::None),
        }
    }

    pub fn new(value: &String) -> Result<Value, Error> {
        let rv = {
            Value::new_bool(value).unwrap_or(
        Value::new_int(value).unwrap_or(
            Value::new_float(value).unwrap_or(
                Value::new_string(value).unwrap_or(Value::None)
                    )
                )
            )
        };

        Ok(rv)
    }

    pub fn new_bool(value: &String) -> Result<Value, Error> {
        let rv = {
            match value.as_str() {
                "true" => Value::Bool(true),
                "false" => Value::Bool(false),
                _ => return Err(Error {message: "unable to parse Value::Bool".to_string()}),
            }
        };

        Ok(rv)
    }

    pub fn new_int(value: &String) -> Result<Value, Error> {
        match value.parse::<i64>() {
            Ok(i) => Ok(Value::Int(i)),
            Err(_) => Err(Error {message: "unable to parse Value::Int".to_string()}),
        }
    }
    
    pub fn new_float(value: &String) -> Result<Value, Error> {
        match value.parse::<f64>() {
            Ok(i) => Ok(Value::Float(i)),
            Err(_) => Err(Error {message: "unable to parse Value::Int".to_string()}),
        }
    }
    
    pub fn new_string(value: &String) -> Result<Value, Error> {
        Ok(Value::String(value.clone()))
    }
    
    pub fn new_vec(value: &String) -> Result<Value, Error> {
        let rv= {
            value.split(',').map(|val| {
                Value::new(&val.to_string()).unwrap()
            })
            .collect()
        };

        Ok(Value::Vec(rv))
    }
    
    pub fn new_map(value: &String) -> Result<Value, Error> {
        let vect = {
            value.split(',').map(|val| {
                val.split_once(':')
            })
            .collect::<Vec<Option<(&str, &str)>>>()
        };

        let mut rv = HashMap::new();

        for p in vect {
            if p.is_none() {
                return Err(Error {message: "unable to parse Value::Map".to_string()})
            }

            let (k, v) = p.unwrap();
            rv.insert(k.to_string(), Value::new(&v.to_string()).unwrap());
        }

        Ok(Value::Map(rv))
    }

    pub fn type_of(&self) -> ValueType {
        match self {
            Value::Bool(_) => ValueType::Bool,
            Value::Int(_) => ValueType::Int,
            Value::Float(_) => ValueType::Float,
            Value::String(_) => ValueType::String,
            Value::Vec(_) => ValueType::Vec,
            Value::Map(_) => ValueType::Map,
            _ => ValueType::None,
        }
    }

    // TODO: implement all casts

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(s) => Some(*s),
            _ => None,
        }
    }

    pub fn as_int_default(&self, default: i64) -> i64 {
        match self {
            Value::Int(s) => *s,
            _ => default,
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_str_default<'a>(&'a self, default: &'a str) -> &'a str {
        match self {
            Value::String(s) => s,
            _ => default,
        }
    }

    pub fn as_vec(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Vec(v) => Some(&v),
            _ => return None,
        }
    }

    pub fn as_map(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Map(m) => Some(&m),
            _ => return None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt (&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let fmt_str = match self {
            Value::Bool(v) => format!("Bool: {}", v),
            Value::Int(v) => format!("Int: {}", v),
            Value::Float(v) => format!("Float: {}", v),
            Value::String(v) => format!("String: {}", v),
            Value::Vec(_) => "Vec: {}".to_string(),
            Value::Map(_) => "Map: {}".to_string(),
            _ => "None".to_string(),
        };

        fmt.write_str(fmt_str.as_str())
    }
}