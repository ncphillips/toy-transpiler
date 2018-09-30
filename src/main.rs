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


    let code = "def f(x, y, z) 1 end";
    tokenizer::tokenize(code, &mut tokens, &token_kinds);

    let ast = parser::parse(&mut tokens);

    println!("{}", ast);

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
pub struct TokenKind {
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

    pub fn parse<'code>(tokens: &mut Vec<Token<'code>>) -> DefNode<'code> {
        parse_def(tokens)
    }

    pub fn parse_def<'code>(tokens: &mut Vec<Token<'code>>) -> DefNode<'code> {
        consume(tokens, "def").expect("def");

        let name = consume(tokens, "identifier").expect("function name");
        let arg_names = parse_def_args(tokens);
        let body = parse_expr(tokens);

        parse_end(tokens);

        DefNode {
            name: name.value,
            arg_names,
            body,
        }
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

    fn parse_expr(tokens: &mut Vec<Token>) -> Vec<IntNode> {
        parse_int(tokens)
    }

    fn parse_int<'code>(tokens: &mut Vec<Token<'code>>) -> Vec<IntNode> {
        consume(tokens, "integer").expect("body");
        vec![IntNode { value: 1 }]
    }
    


    fn next_is(kind_name: &str, tokens: &Vec<Token>) -> bool {
        &tokens[0].kind.name == kind_name
    }

    pub fn consume<'code>(tokens: &mut Vec<Token<'code>>, kind: &str) -> Result<Token<'code>, String>  {
        let next_token = tokens.remove(0);
        
        if next_token.kind.name == kind {
            return Ok(next_token);
        }

        Err(next_token.kind.name.clone())
    }
}

/// DefNode
pub struct DefNode<'code> {
    name: &'code str,
    arg_names: Vec<&'code str>,
    body: Vec<IntNode>,
}

impl<'code> fmt::Display for DefNode<'code> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "<DefNode name='{}' arg_names={:?} >",
            self.name,
            self.arg_names,
        )
    }
}

/// IntNode
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
pub struct CallNode<'code> {
    name: String,
    arg_expr: Vec<&'code str>,
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


