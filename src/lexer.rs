use core::panic;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Number,
    String,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Equal,
    Eol,
    Type(Type),
    Value(String),
    Var(String),
}

#[derive(Debug)]
pub struct Statement {
    pub tokens: Vec<Token>,
}

impl Statement {
    fn new() -> Self {
        Self { tokens: Vec::new() }
    }
}

pub struct Lexer {
    text: Vec<char>,
    pub statements: Vec<Statement>,
}

impl Lexer {
    pub fn new(text: Vec<char>) -> Self {
        Self {
            text,
            statements: vec![Statement::new()],
        }
    }

    pub fn read(&mut self) {
        self.text.retain(|c| *c != '\n');

        let mut i: usize = 0;
        let mut phrase: String = String::new();

        for c in self.text.iter() {
            let current: &mut Statement = self.statements.get_mut(i).unwrap();

            match c {
                ';' => {
                    current.tokens.push(Token::Value(phrase.clone()));
                    current.tokens.push(Token::Eol);

                    self.statements.push(Statement::new());
                    i += 1;
                    phrase = String::new();
                }
                ':' => {
                    current.tokens.push(Token::Var(phrase.clone()));
                    current.tokens.push(Token::Equal);

                    phrase = String::new();
                }
                '^' => {
                    current.tokens.push(Token::Type(match phrase.as_str() {
                        "num" => Type::Number,
                        "str" => Type::String,
                        _ => panic!("Invalid type definition"),
                    }));

                    phrase = String::new();
                }
                ' ' => continue,
                _ => phrase.push(*c),
            };
        }

        self.statements
            .retain(|statement| !statement.tokens.is_empty());
    }
}
