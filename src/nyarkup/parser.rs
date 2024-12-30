use crate::nyarkup::lexer::Token;

struct Parser {
    token_stream: Vec<Token>,
}

impl Parser {
    fn new(token_stream: Vec<Token>) -> Self {
        Self { token_stream }
    }

    fn current_token(&self) -> Token {
        self.token_stream[0]
    }
    fn peek(&self) -> Token {
        self.token_stream[1]
    }

    fn pop(&mut self) {
        self.token_stream.pop();
    }
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
