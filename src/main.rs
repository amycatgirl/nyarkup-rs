pub mod nyarkup;

fn main() {
    use nyarkup::lexer::Lexer;
    let mut lexer = Lexer::new("== hello world");

    lexer.scan_token();

    println!("{:?}", lexer.tokens);
}
