use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Section,
    Divider,
    Bold,
    Italic,
    Strikethrough,
    Code,
    CodeBlock,
    Title,
    Quote,
}

// Note to self: The Copy trait is a subtrait of Clone. Which means that, to derive
// The Copy trait, we need to _always_ derive Clone first.
#[derive(Debug, Clone, Copy)]
pub struct Token {
    token_type: TokenType,
    start: usize,
    end: Option<usize>,
}

impl Token {
    fn new(token_type: TokenType, start: usize, end: Option<usize>) -> Self {
        Self {
            token_type,
            start,
            end,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:?} ({},{})]",
            self.token_type,
            self.start,
            self.end.unwrap_or(self.start)
        )
    }
}

#[derive(Debug)]
pub struct Lexer {
    start: usize,
    current: usize,

    source: String,
    chars: Vec<char>,

    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        let chars: Vec<char> = code.chars().collect();

        Self {
            start: 0,
            current: 0,
            chars,
            source: code,
            tokens: Vec::new(),
        }
    }

    pub fn scan_token(&mut self) {
        let c = self.next();

        match c {
            '{' | '}' => self.add_token(TokenType::Bold),
            '_' => self.add_token(TokenType::Italic),
            '~' => self.add_token(TokenType::Strikethrough),
            '`' => {
                self.start = self.current - 1;

                if self.match_char('`') {
                    if self.match_char('`') {
                        self.add_token(TokenType::CodeBlock);
                    }
                } else {
                    self.add_token(TokenType::Code)
                }
            }
            '-' => {
                self.start = self.current - 1;
                if self.match_char('-') {
                    if self.match_char('-') {
                        self.add_token(TokenType::Divider);
                    } else if self.peek() == ' ' {
                        self.add_token(TokenType::Section);
                    }
                }
            }
            '=' => {
                self.start = self.current - 1;

                if self.match_char('=') && self.peek() == ' ' {
                    self.add_token(TokenType::Title);
                }
            }

            '>' => {
                if self.behind() == '\n' {
                    self.add_token(TokenType::Quote);
                }
            }
            _ => (),
        }

        // Reset start to index 0
        self.start = 0;
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn next(&mut self) -> char {
        let previous_position = self.current;
        self.current += 1;

        self.chars[previous_position]
    }

    fn behind(&mut self) -> char {
        self.chars[self.current - 2]
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
        self.tokens.push(Token::new(
            token,
            self.start,
            if self.current != self.start {
                Some(self.current - 1)
            } else {
                None
            },
        )); // The reason for this is that current is always 1 char ahead when we call add_token
    }
}
