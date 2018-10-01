extern crate regex;

use regex::Regex;

mod node;

fn main() {
     let token_kinds= vec![
        TokenKind::new("def", r"^(\bdef\b)"),
        TokenKind::new("end", r"^(\bend\b)"),
        TokenKind::new("identifier", r"^(\b[a-zA-Z]+\b)"),
        TokenKind::new("integer", r"^(\b[0-9]+\b)"),
        TokenKind::new("oparam", r"^(\()"),
        TokenKind::new("cparam", r"^(\))"),
        TokenKind::new("comma", r"^(,)"),
    ];

    let mut tokens: Vec<Token> = Vec::new();


    let code = "def f(x, y, z) g(name, 2) end";
    tokenizer::tokenize(code, &mut tokens, &token_kinds);

    let ast = parser::parse(&mut tokens);

    println!("\n\n");
    println!("NP: {}\n\n", code);
    println!("JS: {}", generator::generate(&ast));
    println!("\n\n");

}


/// Tokenizer
mod tokenizer {
    use super::*;
    pub fn tokenize<'code>(code: &'code str, tokens: &mut Vec<Token<'code>>, token_kinds: &'code Vec<TokenKind>) {
        if code.len() > 0 {
            let (more_code, token_option) = get_next_token(code, token_kinds);

            if let Some(token) = token_option {
                tokens.push(token);
            }

            tokenize(more_code, tokens, token_kinds)
        } 
    }



    fn get_next_token<'code>(code: &'code str, token_kinds: &'code Vec<TokenKind>) -> (&'code str, Option<Token<'code>>) {
         for kind in token_kinds.iter() {
            if kind.re.is_match(&code) {
                let cap = kind.re.captures(&code).unwrap();
                let value = String::from(&cap[0]);
                let value = &code[..value.len()];
                let code = &code[value.len()..];

                return (code, Some(Token::new(kind, value)));
            } 
        }

        (&code[1..], None)
    }
}

/// TokenKind
pub struct TokenKind{
    name: String,
    re: Regex,
}

impl TokenKind {
    pub fn new(name: &str, re: &str) -> TokenKind {
        TokenKind { 
            name: String::from(name), 
            re: Regex::new(re).unwrap(),
        }
    }
}

/// Token
pub struct Token<'code> {
    kind: &'code TokenKind,
    value: &'code str,
}

impl<'code> Token<'code> {
    fn new(kind: &'code TokenKind, value: &'code str) -> Token<'code> {
        Token { kind, value }
    }
}

/// parser
mod parser {
    use super::*;

    pub fn parse<'code>(tokens: &mut Vec<Token<'code>>) -> node::Node<'code> {
        parse_def(tokens)
    }

    pub fn parse_def<'code>(tokens: &mut Vec<Token<'code>>) -> node::Node<'code> {
        consume(tokens, "def").expect("def");

        let name = consume(tokens, "identifier").expect("function name");
        let arg_names = parse_def_args(tokens);
        let body = vec![parse_expr(tokens)];

        parse_end(tokens);

        node::Node::Def(node::DefNode {
            name: name.value,
            arg_names,
            body,
        })
    }

    fn parse_def_args<'code>(tokens: &mut Vec<Token<'code>>) -> Vec<&'code str> {
        let mut def_args = Vec::new();

        consume(tokens, "oparam")
            .expect("Expected an \"oparam\" but received");

        while !next_is("cparam", tokens) {
            let token = consume(tokens, "identifier").expect("next arg");
            def_args.push(token.value);

            if !next_is("cparam", tokens) {
                consume(tokens, "comma")
                    .expect("Expected a \"comma\" but received");
            }
        };
        
        consume(tokens, "cparam")
            .expect("Expected an \"cparam\" but received");

        def_args
    }

    fn parse_end<'code>(tokens: &mut Vec<Token<'code>>) {
        consume(tokens, "end").expect("end");
    }

    fn parse_expr<'code>(tokens: &mut Vec<Token<'code>>) -> node::Node<'code> {
        if next_is("integer", tokens) {
            parse_int(tokens)
        } else if next_is("identifier", tokens) && index_is(1, "oparam", tokens) {
            parse_call(tokens)
        } else {
            parse_var_ref(tokens)
        }
    }

    fn parse_var_ref<'code>(tokens: &mut Vec<Token<'code>>) -> node::Node<'code> {
        let token = consume(tokens, "identifier").expect("variable");
        let name = token.value;
        node::Node::VarRef(node::VarRefNode { name })
    }

    fn parse_int<'code>(tokens: &mut Vec<Token<'code>>) -> node::Node<'code> {
        let token = consume(tokens, "integer").expect("body");
        let value: i32 = token.value.parse().unwrap();
        node::Node::Int(node::IntNode { value })
    }

    fn parse_call<'code>(tokens: &mut Vec<Token<'code>>) -> node::Node<'code> {
        let name = consume(tokens, "identifier").expect("identifier");
        let name = String::from(name.value);
        let arg_expr = parse_call_args(tokens);
        node::Node::Call(node::CallNode { name, arg_expr })
    }

    fn parse_call_args<'code>(tokens: &mut Vec<Token<'code>>) -> Vec<node::Node<'code>> {
        let mut call_args = Vec::new();

        consume(tokens, "oparam")
            .expect("Expected an \"oparam\" but received");

        while !next_is("cparam", tokens) {
            let node = parse_expr(tokens);
            call_args.push(node);

            if !next_is("cparam", tokens) {
                consume(tokens, "comma")
                    .expect("Expected a \"comma\" but received");
            }
        };
        
        consume(tokens, "cparam")
            .expect("Expected an \"cparam\" but received");

        call_args
    }

    fn next_is(kind_name: &str, tokens: &Vec<Token>) -> bool {
        index_is(0, kind_name, tokens)
    }

    fn index_is(index: usize, kind_name: &str, tokens: &Vec<Token>) -> bool {
        &tokens[index].kind.name == kind_name
    }

    pub fn consume<'code>(tokens: &mut Vec<Token<'code>>, kind: &str) -> Result<Token<'code>, String>  {
        let next_token = tokens.remove(0);
        
        if next_token.kind.name == kind {
            return Ok(next_token);
        }

        Err(next_token.kind.name.clone())
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
