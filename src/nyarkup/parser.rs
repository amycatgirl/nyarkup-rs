use std::any::Any;

use super::lexer::{Token, TokenType};

struct Parser {
    token_stream: Vec<Token>,
    cursor: usize,
}

impl Parser {
    fn new(token_stream: Vec<Token>) -> Self {
        Self {
            token_stream,
            cursor: 0,
        }
    }

    fn parse_document(&mut self) -> bool {
        self.parse_expression() && self.expect_token(TokenType::EOF)
    }

    fn parse_expression(&self) -> bool {}

    fn parse_headers() -> bool {}

    fn parse_formatting() -> bool {}

    fn parse_bold() -> bool {}
    fn parse_italic() -> bool {}
    fn parse_strikethrough() -> bool {}
    fn parse_code() -> bool {}

    fn scan_token(&mut self) -> Token {
        let current_token: Token = self.token_stream[self.cursor];
        self.cursor += 1;

        return current_token;
    }

    fn putback_token(&mut self) {
        self.cursor -= 1;
    }

    fn expect_token(&mut self, expected: TokenType) -> bool {
        let token = self.scan_token();

        if token.token_type.type_id() == expected.type_id() {
            true
        } else {
            self.putback_token();
            false
        }
    }

    // [TODO] Errors!

    // Language design is a time-consuming task, but it's soooooooo worth it

    /*
     * [INFO] I suck at making LL(1) grammar, it's my first time
     * So um, have it if you want to request some changes or smth
     * D -> E
     * E -> E'
     * E' -> T
     * E' -> {E'}
     * E' -> _E'_
     * E' -> ~E'~
     * E' -> > E'
     * E' -> `e`
     * E' -> e
     * T -> == e
     * T -> -- e
     */
}
