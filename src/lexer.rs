use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// TODO: CONVERT TO STRUCT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub name: String,
    pub index: usize,
    pub length: Option<usize>,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Lexer {
    start: usize,
    current: usize,
    source: String,
    chars: Vec<char>,
    tokens: Vec<Token>,
}

#[wasm_bindgen]
impl Lexer {
    #[wasm_bindgen(constructor)]
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
        let c = self.current();

        match c {
            '{' => {
                self.add_token(Token {
                    name: "BoldStart".to_string(),
                    index: self.current,
                    length: None,
                });
            }
            '}' => {
                self.add_token(Token {
                    name: "BoldEnd".to_string(),
                    index: self.current,
                    length: None,
                });
            }
            '_' => {
                self.add_token(Token {
                    name: "Italic".to_string(),
                    index: self.current,
                    length: None,
                });
            }
            '~' => {
                self.add_token(Token {
                    name: "Strikethrough".to_string(),
                    index: self.current,
                    length: None,
                });
            }
            '\n' => {
                self.add_token(Token {
                    name: "Strikethrough".to_string(),
                    index: self.current,
                    length: None,
                });
            }
            '`' => {
                self.add_token(Token {
                    name: "InlineCode".to_string(),
                    index: self.current,
                    length: None,
                });
            }
            '>' => {
                self.add_token(Token {
                    name: "Quote".to_string(),
                    index: self.current,
                    length: None,
                });
            }
            '=' => {
                self.start = self.current;
                match self.match_char('=') {
                    true => {
                        self.add_token(Token {
                            name: "Title".to_string(),
                            index: self.current,
                            length: Some(self.current - self.start),
                        });
                    }
                    false => (),
                }
            }
            '-' => {
                self.start = self.current;
                match self.match_char('-') {
                    true => {
                        if self.peek() == '-' {
                            self.add_token(Token {
                                name: "HorizontalRule".to_string(),
                                index: self.current,
                                length: Some((self.current + 1) - self.start),
                            });
                        }

                        self.add_token(Token {
                            name: "Section".to_string(),
                            index: self.current,
                            length: Some(self.current - self.start),
                        });
                    }
                    false => (),
                }
            }
            _ => (),
        }

        // Reset start to index 0
        self.start = 0;
        self.current += 1;
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn current(&mut self) -> char {
        let char = self.chars[self.current];
        return char;
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

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token); // The reason for this is that current is always 1 char ahead when we call add_token
    }
}
