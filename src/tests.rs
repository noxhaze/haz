use std::collections::HashMap;

use crate::config::Config;
use crate::lexer::{Token, Type};
use crate::parser::Value;

#[test]
fn creation() {
    let config: std::io::Result<Config> = Config::new("test/creation.haz");
    assert!(config.is_ok());
}

#[test]
fn read() {
    let config = Config::new("test/read.haz").unwrap();
    let tokens = vec![
        Token::Type(Type::String),
        Token::Var(String::from("test_var")),
        Token::Equal,
        Token::Value(String::from("test_value")),
        Token::Eol,
    ];

    assert_eq!(tokens, config.lexer.statements.get(0).unwrap().tokens);
}

#[test]
fn read_with_space() {
    let config = Config::new("test/read_with_space.haz").unwrap();
    let tokens = vec![
        Token::Type(Type::String),
        Token::Var(String::from("test_var")),
        Token::Equal,
        Token::Value(String::from("test_value")),
        Token::Eol,
    ];

    assert_eq!(tokens, config.lexer.statements.get(0).unwrap().tokens);
}

#[test]
fn read_types_number() {
    let config = Config::new("test/read_types_number.haz").unwrap();
    let tokens = vec![
        Token::Type(Type::Number),
        Token::Var(String::from("test_var")),
        Token::Equal,
        Token::Value(String::from("10")),
        Token::Eol,
    ];

    assert_eq!(tokens, config.lexer.statements.get(0).unwrap().tokens);
}

#[test]
fn read_types_string() {
    let config = Config::new("test/read_types_string.haz").unwrap();
    let tokens = vec![
        Token::Type(Type::String),
        Token::Var(String::from("test_var")),
        Token::Equal,
        Token::Value(String::from("test_value")),
        Token::Eol,
    ];

    assert_eq!(tokens, config.lexer.statements.get(0).unwrap().tokens);
}

#[test]
fn parse() {
    let config = Config::new("test/parse.haz").unwrap();
    let vars: HashMap<String, Value> = HashMap::from([
        ("test_num".to_string(), Value::Number(10)),
        ("test_str".to_string(), Value::String(String::from("hello"))),
        ("test_bool".to_string(), Value::Bool(true)),
    ]);

    assert_eq!(vars, config.parser.values);
}
