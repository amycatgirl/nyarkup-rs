use std::fmt::Display;

#[derive(Debug)]
pub enum TokenType {
    Bold(Vec<char>),
    Italic(Vec<char>),
    Strikethrough(Vec<char>),
    Code(Vec<char>),
    Codeblock(Vec<char>),
    Title(Vec<char>),
    Section(Vec<char>),
    EOF,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, line: usize) -> Self {
        Self { token_type, line }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.token_type, self.line)
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
            '=' => {
                if self.match_char('=') {
                    self.next();
                    let started_at = self.current;

                    while self.peek() != '\n' && !self.is_at_end() {
                        self.next();
                    }

                    self.add_token(TokenType::Title(
                        self.chars[started_at..self.current].to_vec(),
                    ));
                }
            }

            // Ignore whitespace
            ' ' | '\r' | '\t' => (),

            '\n' => self.line += 1,

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

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.chars[self.current] != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.chars[self.current]
    }

    fn add_token(&mut self, token: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token, self.line));
    }
}
