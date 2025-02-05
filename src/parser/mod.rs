pub mod errors;

use super::lexer::Token;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[wasm_bindgen]
pub enum NodeType {
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

#[derive(Serialize, Deserialize, Debug)]
#[wasm_bindgen]
struct Node {
    pub r#type: NodeType,
    children: Option<Vec<Box<Node>>>,
    pub length: Option<usize>,
    pub starts_at: Option<usize>,
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

    pub fn parse(&mut self) -> Result<JsValue, errors::ParseError> {
        let mut nodes = vec![];

        while !self.is_out_of_tokens() {
            console::log_1(&format!("Tokens: {:?}", self.token_stream).into());
            match self.consume_title() {
                Some(n) => {
                    console::log_1(&"title node found".into());
                    nodes.push(Box::new(n))
                }
                None => (),
            }
        }

        let root_node = Node {
            r#type: NodeType::Root,
            children: Some(nodes),
            length: None,
            starts_at: None,
        };
        console::log_1(&format!("{:#?}", root_node).into());

        Ok(serde_wasm_bindgen::to_value(&root_node).unwrap())
    }

    fn consume_title(&mut self) -> Option<Node> {
        let token = self.scan_token().unwrap();

        console::log_1(&format!("{:?}", token).into());

        if token.name == "Title".to_string() {
            console::log_1(&"title node".into());
            match self.consume_new_line() {
                Some(t) => Some(Node {
                    r#type: NodeType::Title,
                    children: None,
                    length: Some(t.index - token.index),
                    starts_at: Some(token.index),
                }),
                None => None,
            }
        } else {
            None
        }
    }

    fn is_out_of_tokens(&self) -> bool {
        self.token_stream.len() <= 0
    }

    fn consume_new_line(&mut self) -> Option<Token> {
        let token = self.scan_token().unwrap();

        console::log_1(&format!("{:?}", token).into());

        if token.name == "NewLine".to_string() {
            console::log_1(&"new line node".into());
            Some(token)
        } else {
            self.putback_token(token);

            None
        }
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
