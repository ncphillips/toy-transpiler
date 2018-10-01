extern crate regex;

mod node;
mod parser;
mod token;

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


/// token::Tokenizer
mod tokenizer {
    use super::*;
    pub fn tokenize<'code>(code: &'code str, tokens: &mut Vec<token::Token<'code>>, token_kinds: &'code Vec<token::TokenKind>) {
        if code.len() > 0 {
            let (more_code, token_option) = get_next_token(code, token_kinds);

            if let Some(token) = token_option {
                tokens.push(token);
            }

            tokenize(more_code, tokens, token_kinds)
        } 
    }



    fn get_next_token<'code>(code: &'code str, token_kinds: &'code Vec<token::TokenKind>) -> (&'code str, Option<token::Token<'code>>) {
         for kind in token_kinds.iter() {
            if kind.re.is_match(&code) {
                let cap = kind.re.captures(&code).unwrap();
                let value = String::from(&cap[0]);
                let value = &code[..value.len()];
                let code = &code[value.len()..];

                return (code, Some(token::Token::new(kind, value)));
            } 
        }

        (&code[1..], None)
    }
}

/// Generator

mod generator {
    use super::*;

    pub fn generate(node: &node::Node) -> String {
        match node {
            node::Node::Def(def_node) => generate_def(def_node),
            node::Node::Call(call_node) => generate_call(call_node),
            node::Node::Int(int_node) => generate_int(int_node),
            node::Node::VarRef(var_ref_node) => generate_var_ref(var_ref_node),
        }
    }

    fn generate_def(def_node: &node::DefNode) -> String {
        let mut body_expr = Vec::new();
        for b in &def_node.body {
            body_expr.push(generate(&b));
        };
        format!(
            "function {}({}) {{ return {} }}",
            def_node.name,
            def_node.arg_names.join(", "),
            body_expr.join(";")
        )
    }

    fn generate_call(call_node: &node::CallNode) -> String {
        let mut arg_expr = Vec::new();
        for expr in &call_node.arg_expr {
            arg_expr.push(generate(&expr));
        };
        format!(
            "{}({})",
            call_node.name,
            arg_expr.join(", ")
        )
    }

    fn generate_int(int_node: &node::IntNode) -> String {
        format!("{}", int_node.value)
    }

    fn generate_var_ref(var_ref_node: &node::VarRefNode) -> String {
        format!("{}", var_ref_node.name)
    }

}
