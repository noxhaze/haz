use crate::config::Config;
use crate::lexer::Token;

#[test]
fn creation() {
    let config: std::io::Result<Config> = Config::new("test/creation.haz");
    assert!(config.is_ok());
}

#[test]
fn read() {
    let config = Config::new("test/read.haz").unwrap();
    let tokens = vec![
        Token::Var(String::from("test_var")),
        Token::Equal(':'),
        Token::Value(String::from("test_value")),
        Token::Eol(';'),
    ];

    assert_eq!(tokens, config.lexer.statements.get(0).unwrap().tokens);
}

#[test]
fn read_with_space() {
    let config = Config::new("test/read_with_space.haz").unwrap();
    let tokens = vec![
        Token::Var(String::from("test_var")),
        Token::Equal(':'),
        Token::Value(String::from("test_value")),
        Token::Eol(';'),
    ];

    assert_eq!(tokens, config.lexer.statements.get(0).unwrap().tokens);
}
