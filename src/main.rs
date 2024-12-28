use std::fs;
pub mod nyarkup;

fn main() {
    use nyarkup::lexer::Lexer;
    let file_contents = fs::read_to_string("./test.nya").expect("Where the fuck is my file");
    let mut lexer = Lexer::new(file_contents);

    while !lexer.is_at_end() {
        lexer.scan_token();
    }

    println!("Tokens:");

    for token in lexer.tokens.iter() {
        print!("{} ", token);
    }
}
