use std::{fs::File, io::Read};

use crate::lexer::Lexer;
use crate::parser::Parser;

pub struct Config {
    pub file: File,
    pub lexer: Lexer,
    pub parser: Parser,
}

impl Config {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;
        let text = text.chars().collect();
        let mut lexer = Lexer::new(text);
        lexer.read();

        let mut parser = Parser::new();
        parser.parse_values(&lexer.statements);

        Ok(Self {
            file,
            lexer,
            parser,
        })
    }
}
