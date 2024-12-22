pub mod nyarkup;

fn main() {
    use nyarkup::lexer::Lexer;
    let mut lexer = Lexer::new("*~_");

    lexer.scan_token();
    lexer.scan_token();
    lexer.scan_token();

    println!("{:?}", lexer.tokens);
}
