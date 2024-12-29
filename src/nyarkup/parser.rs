use crate::nyarkup::lexer::Token;

struct Parser {
    token_stream: Vec<Token>,
}

impl Parser {
    fn peek(&self) -> Token {
        self.token_stream[1]
    }

    fn pop(&mut self) {
        self.token_stream.pop();
    }

    // todo)) Document grammar for recursive-desent parsing!!
    // Language design is a time-consuming task, but it's soooooooo worth it
}
