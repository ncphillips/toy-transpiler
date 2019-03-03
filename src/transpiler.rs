mod generator;
mod node;
mod parser;
mod token;
mod tokenizer;

pub fn transpile<'code>(code: &'code str) -> String {
    let token_kinds = vec![
        token::TokenKind::new("def", r"^(\bdef\b)"),
        token::TokenKind::new("end", r"^(\bend\b)"),
        token::TokenKind::new("identifier", r"^(\b[a-zA-Z]+\b)"),
        token::TokenKind::new("integer", r"^(\b[0-9]+\b)"),
        token::TokenKind::new("oparam", r"^(\()"),
        token::TokenKind::new("cparam", r"^(\))"),
        token::TokenKind::new("comma", r"^(,)"),
    ];

    let mut tokens: Vec<token::Token> = Vec::new();

    tokenizer::tokenize(code, &mut tokens, &token_kinds);

    let ast = parser::parse(&mut tokens);

    return generator::generate(&ast);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handles_empty_code() {
        let input_code = "";

        let output_code = "";

        assert_eq!(output_code, transpile(input_code));
    }

    #[test]
    fn it_handles_empty_functions() {
        let input_code = "def a() end";

        let output_code = "function a() { }";

        assert_eq!(output_code, transpile(input_code));
    }

    #[test]
    fn it_transpiles() {
        let input_code = "def f(x, y, z) g(name, 2) end";

        let output_code = "function f(x, y, z) { return g(name, 2) }";

        assert_eq!(output_code, transpile(input_code));
    }

    #[test]
    fn it_handles_function_that_returns_a_constant_number() {
        let input_code = "def f() 1 end";

        let output_code = "function f() { return 1 }";

        assert_eq!(output_code, transpile(input_code));
    }
}
