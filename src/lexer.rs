#[derive(Debug, PartialEq)]
pub enum Token {
    Equal(char),
    Eol(char),
    Value(String),
    Var(String),
}

#[derive(Debug)]
pub struct Statement {
    pub tokens: Vec<Token>,
}

impl Statement {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }
}

pub struct Lexer {
    pub text: Vec<char>,
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
        let mut i: usize = 0;
        let mut phrase: String = String::new();

        for c in self.text.iter() {
            let current: &mut Statement = self.statements.get_mut(i).unwrap();

            match c {
                ';' => {
                    current.tokens.push(Token::Value(phrase.clone()));
                    current.tokens.push(Token::Eol(';'));

                    self.statements.push(Statement::new());
                    i += 1;
                    phrase = String::new();
                }
                ':' => {
                    current.tokens.push(Token::Var(phrase.clone()));
                    current.tokens.push(Token::Equal(':'));

                    phrase = String::new();
                }
                _ => phrase.push(*c),
            };
        }
    }
}
