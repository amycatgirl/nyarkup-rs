use nyarkup_wasm::lexer::Lexer;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn lex_title() {
    let test_string = r#"
            == Hello World!
        "#
    .to_string();

    let mut lexer = Lexer::new(test_string.clone());

    lexer.lex();
}
