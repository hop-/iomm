use crate::config::{
    error::Error,
    value::{Value, ValueType},
};


#[derive(PartialEq)]
pub enum Require {
    True,
    False,
    Default(Value),
}
pub struct Attribute {
    pub name: String,
    pub var: Option<String>,
    pub value_type: ValueType,
    pub required: Require,
}

impl Attribute {
    pub fn new(name: &str, var: Option<&str>, required: Require, value_type: ValueType) -> Result<Attribute, Error> {
        let var = if var == None { None } else { Some(var.unwrap().to_string()) };

        match &required {
            Require::Default(v) => {
                if v.type_of() != value_type {
                    return Err(Error {message: "wrong default type".to_string()});
                }
            },
            _ => (),
        }

        Ok(
            Attribute {
                name: name.to_string(),
                var: var,
                required: required,
                value_type: value_type,
            }
        )
    }
}