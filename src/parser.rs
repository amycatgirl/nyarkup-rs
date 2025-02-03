use super::lexer::Token;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Parser {
    token_stream: Vec<Token>,
}

#[wasm_bindgen]
impl Parser {
    #[wasm_bindgen(constructor)]
    pub fn new(token_stream: JsValue) -> Self {
        let mut ts: Vec<Token> = serde_wasm_bindgen::from_value(token_stream).unwrap();
        ts.reverse();

        Self { token_stream: ts }
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
        match token {
            Some(t) => {
                if t.name == "Title".to_string() {
                    match self.lookahead().name.as_str() {
                        "NewLine" => true,
                        _ => false,
                    }
                } else {
                    self.putback_token(t);
                    false
                }
            }
            None => false,
        }
    }

    fn eat_text(&mut self) -> bool {
        return true;
    }

    fn scan_token(&mut self) -> Option<Token> {
        self.token_stream.pop()
    }

    fn lookahead(&self) -> Token {
        return self
            .token_stream
            .get(self.token_stream.len() - 1)
            .unwrap()
            .clone();
    }

    fn putback_token(&mut self, token: Token) {
        self.token_stream.push(token);
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
