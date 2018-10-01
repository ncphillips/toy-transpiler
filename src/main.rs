extern crate regex;

mod node;
mod parser;
mod token;
mod tokenizer;
mod generator;


fn main() {
     let token_kinds= vec![
        token::TokenKind::new("def", r"^(\bdef\b)"),
        token::TokenKind::new("end", r"^(\bend\b)"),
        token::TokenKind::new("identifier", r"^(\b[a-zA-Z]+\b)"),
        token::TokenKind::new("integer", r"^(\b[0-9]+\b)"),
        token::TokenKind::new("oparam", r"^(\()"),
        token::TokenKind::new("cparam", r"^(\))"),
        token::TokenKind::new("comma", r"^(,)"),
    ];

    let mut tokens: Vec<token::Token> = Vec::new();


    let code = "def f(x, y, z) g(name, 2) end";
    tokenizer::tokenize(code, &mut tokens, &token_kinds);

    let ast = parser::parse(&mut tokens);

    println!("\n\n");
    println!("NP: {}\n\n", code);
    println!("JS: {}", generator::generate(&ast));
    println!("\n\n");

}
