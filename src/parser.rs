use super::lexer::Token;

pub struct Parser {
    token_stream: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(token_stream: Vec<Token>) -> Self {
        Self {
            token_stream,
            cursor: 0,
        }
    }

    pub fn validate_content(&mut self) -> bool {
        return self.eat();
    }

    fn eat(&mut self) -> bool {
        self.eat_document()
    }

    fn eat_document(&mut self) -> bool {
        self.eat_title() && self.eat_text()
    }

    fn eat_title(&mut self) -> bool {
        let token = self.scan_token();
        if token.name == "Title".to_string() {
            true
        } else {
            self.putback_token();
            false
        }
    }

    fn eat_text(&mut self) -> bool {
        return true;
    }

    fn scan_token(&mut self) -> Token {
        let current_token: &Token = &self.token_stream[self.cursor];
        self.cursor += 1;

        return current_token.clone();
    }

    fn putback_token(&mut self) {
        self.cursor -= 1;
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
