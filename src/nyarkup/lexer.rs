use std::fmt::Display;

#[derive(Debug)]
pub enum TokenType {
    Bold,
    Italic,
    Strikethrough,
    Code,
    Codeblock,
    Title,
    Section,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, self.line)
    }
}

#[derive(Debug)]
pub struct Lexer {
    start: usize,
    current: usize,
    line: usize,

    source: String,
    chars: Vec<char>,

    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(code: &str) -> Self {
        let chars: Vec<char> = code.chars().collect();

        Self {
            start: 0,
            current: 0,
            line: 1,
            chars,
            source: code.to_string(),
            tokens: Vec::new(),
        }
    }

    pub fn scan_token(&mut self) {
        let c = self.next();

        match c {
            '*' => self.add_token(TokenType::Bold),
            '_' => self.add_token(TokenType::Italic),
            '~' => self.add_token(TokenType::Strikethrough),
            _ => panic!("fuck"),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn next(&mut self) -> char {
        let previous_position = self.current;
        self.current += 1;

        self.chars[previous_position]
    }

    fn add_token(&mut self, token: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token, text, self.line));
    }
}
