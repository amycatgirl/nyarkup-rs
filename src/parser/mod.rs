pub mod errors;

use super::lexer::Token;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
enum NodeType {
    Root = 0,
    Title = 1,
    Section = 2,
    Bold = 3,
    Italic = 4,
    Strikethrough = 5,
    InlineCode = 6,
    CodeBlock = 7,
    Quote = 8,
}

#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
struct Node {
    r#type: NodeType,
    children: Option<Vec<Box<Node>>>,
    length: usize,
    starts_at: usize,
}
#[wasm_bindgen]
impl Node {
    pub fn get_children(&self) -> Result<JsValue, errors::NoChildren> {
        match &self.children {
            Some(value) => Ok(serde_wasm_bindgen::to_value(&value).unwrap()),
            None => Err(errors::NoChildren),
        }
    }
}

#[wasm_bindgen]
pub struct Parser {
    token_stream: Vec<Token>,
    source: String,
}

#[wasm_bindgen]
impl Parser {
    #[wasm_bindgen(constructor)]
    pub fn new(token_stream: JsValue, source: String) -> Self {
        let mut ts: Vec<Token> = serde_wasm_bindgen::from_value(token_stream).unwrap();
        ts.reverse();

        Self {
            token_stream: ts,
            source,
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

    fn eat_bold(&mut self) -> bool {
        let token = self.scan_token();
        let lookahead = self.lookahead();
        match token {
            Some(t) => {
                if t.name == "BoldStart".to_string() && lookahead.name == "BoldEnd".to_string() {
                    true
                } else {
                    self.putback_token(t);
                    false
                }
            }
            None => false,
        }
    }

    fn eat_text(&mut self) -> bool {
        return self.eat_bold();
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
