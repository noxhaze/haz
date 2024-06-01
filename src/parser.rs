use crate::lexer::{Statement, Token, Type};
use std::collections::hash_map::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(i32),
    Bool(bool),
}

pub struct Parser {
    pub values: HashMap<String, Value>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn parse_values(&mut self, statements: &Vec<Statement>) {
        for statement in statements.iter() {
            let mut var_name = String::new();
            let mut var_def = String::new();
            let mut var_type: Option<Type> = None;
            for token in statement.tokens.iter() {
                match token {
                    Token::Var(val) => var_name = val.to_string(),
                    Token::Value(val) => var_def = val.to_string(),
                    Token::Type(val) => var_type = Some(val.clone()),
                    _ => continue,
                };
            }

            println!("{:?}", var_type);
            let var_def = match var_type.expect("No type definition was given to variable") {
                Type::String => Value::String(var_def),
                Type::Number => Value::Number(
                    var_def
                        .parse()
                        .expect("Failed to parse variable with type `Number` into i32"),
                ),
                Type::Bool => Value::Bool(
                    var_def
                        .parse()
                        .expect("Failed to parse variable with type `Bool` into bool"),
                ),
            };

            self.values.insert(var_name, var_def);
        }
    }
}
