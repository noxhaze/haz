enum Token {
    Equal(char),
    Eol(char),
    Value(String),
    Var(String),
}

pub struct Statement {
    tokens: Vec<Token>,
}

impl Statement {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }
}

pub struct Lexer {
    text: Vec<char>,
    statements: Vec<Statement>,
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

                    Self::reset(&mut self.statements, &mut phrase, &mut i);
                }
                ':' => {
                    current.tokens.push(Token::Var(phrase.clone()));
                    current.tokens.push(Token::Equal(':'));

                    Self::reset(&mut self.statements, &mut phrase, &mut i);
                }
                _ => phrase.push(*c),
            };
        }
    }

    fn reset(statements: &mut Vec<Statement>, phrase: &mut String, i: &mut usize) {
        *phrase = String::new();
        statements.push(Statement::new());
        *i += 1;
    }
}
