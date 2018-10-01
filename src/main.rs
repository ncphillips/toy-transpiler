extern crate regex;

use std::fmt;
use regex::Regex;

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

    pub fn parse<'code>(tokens: &mut Vec<Token<'code>>) -> Node<'code> {
        parse_def(tokens)
    }

    pub fn parse_def<'code>(tokens: &mut Vec<Token<'code>>) -> Node<'code> {
        consume(tokens, "def").expect("def");

        let name = consume(tokens, "identifier").expect("function name");
        let arg_names = parse_def_args(tokens);
        let body = vec![parse_expr(tokens)];

        parse_end(tokens);

        Node::Def(DefNode {
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

    fn parse_expr<'code>(tokens: &mut Vec<Token<'code>>) -> Node<'code> {
        if next_is("integer", tokens) {
            parse_int(tokens)
        } else if next_is("identifier", tokens) && index_is(1, "oparam", tokens) {
            parse_call(tokens)
        } else {
            parse_var_ref(tokens)
        }
    }

    fn parse_var_ref<'code>(tokens: &mut Vec<Token<'code>>) -> Node<'code> {
        let token = consume(tokens, "identifier").expect("variable");
        let name = token.value;
        Node::VarRef(VarRefNode { name })
    }

    fn parse_int<'code>(tokens: &mut Vec<Token<'code>>) -> Node<'code> {
        let token = consume(tokens, "integer").expect("body");
        let value: i32 = token.value.parse().unwrap();
        Node::Int(IntNode { value })
    }

    fn parse_call<'code>(tokens: &mut Vec<Token<'code>>) -> Node<'code> {
        let name = consume(tokens, "identifier").expect("identifier");
        let name = String::from(name.value);
        let arg_expr = parse_call_args(tokens);
        Node::Call(CallNode { name, arg_expr })
    }

    fn parse_call_args<'code>(tokens: &mut Vec<Token<'code>>) -> Vec<Node<'code>> {
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

/// Node
#[derive(Debug)]
pub enum Node<'code> {
    Def(DefNode<'code>),
    Int(IntNode),
    Call(CallNode<'code>),
    VarRef(VarRefNode<'code>),
}

/// DefNode
#[derive(Debug)]
pub struct DefNode<'code> {
    name: &'code str,
    arg_names: Vec<&'code str>,
    body: Vec<Node<'code>>,
}

impl<'code> fmt::Display for DefNode<'code> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "<DefNode name='{}' arg_names={:?} body={:?}>",
            self.name,
            self.arg_names,
            self.body,
        )
    }
}
/// IntNode
#[derive(Debug)]
pub struct IntNode {
    value: i32,
}

impl fmt::Display for IntNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "<IntNode value={} >",
            self.value,
        )
    }
}

/// CallNode
#[derive(Debug)]
pub struct CallNode<'code> {
    name: String,
    arg_expr: Vec<Node<'code>>,
}

impl<'code> fmt::Display for CallNode<'code> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "<CallNode name='{}' >",
            self.name,
        )
    }
}

/// VarRefNode
#[derive(Debug)]
pub struct VarRefNode<'code> {
    name: &'code str,
}

impl<'code> fmt::Display for VarRefNode<'code> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "<VarRefNode name='{}' >",
            self.name,
        )
    }
}








/// Generator

mod generator {
    use super::*;

    pub fn generate(node: &Node) -> String {
        match node {
            Node::Def(n) => {
                let mut bodies = String::from("");
                for b in &n.body {
                    bodies = generate(&b);
                };
                format!(
                    "function {}({}) {{ return {} }}",
                    n.name,
                    n.arg_names.join(", "),
                    bodies
                )
            }
            _ =>  String::from(""),
        }
    }

}
