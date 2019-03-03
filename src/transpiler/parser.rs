use super::node::{CallNode, DefNode, IntNode, Node, RootNode, VarRefNode};
use super::token::Token;

pub fn parse<'code>(tokens: &mut Vec<Token<'code>>) -> Node<'code> {
    let mut body = Vec::new();

    if !tokens.is_empty() {
        body.push(parse_def(tokens))
    }

    Node::Root(RootNode { body })
}

fn parse_def<'code>(tokens: &mut Vec<Token<'code>>) -> Node<'code> {
    consume(tokens, "def").expect("def");

    let name = consume(tokens, "identifier").expect("function name");
    let arg_names = parse_def_args(tokens);
    let mut body = Vec::new();

    if !next_is("end", tokens) {
        body.push(parse_expr(tokens));
    }

    parse_end(tokens);

    Node::Def(DefNode {
        name: name.value,
        arg_names,
        body,
    })
}

fn parse_def_args<'code>(tokens: &mut Vec<Token<'code>>) -> Vec<&'code str> {
    let mut def_args = Vec::new();

    consume(tokens, "oparam").expect("Expected an \"oparam\" but received");

    while !next_is("cparam", tokens) {
        let token = consume(tokens, "identifier").expect("next arg");
        def_args.push(token.value);

        if !next_is("cparam", tokens) {
            consume(tokens, "comma").expect("Expected a \"comma\" but received");
        }
    }

    consume(tokens, "cparam").expect("Expected an \"cparam\" but received");

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

    consume(tokens, "oparam").expect("Expected an \"oparam\" but received");

    while !next_is("cparam", tokens) {
        let node = parse_expr(tokens);
        call_args.push(node);

        if !next_is("cparam", tokens) {
            consume(tokens, "comma").expect("Expected a \"comma\" but received");
        }
    }

    consume(tokens, "cparam").expect("Expected an \"cparam\" but received");

    call_args
}

fn next_is(kind_name: &str, tokens: &[Token]) -> bool {
    index_is(0, kind_name, tokens)
}

// TODO: Pass in enum, not string.
fn index_is(index: usize, kind_name: &str, tokens: &[Token]) -> bool {
    tokens[index].kind.name == kind_name
}

fn consume<'code>(tokens: &mut Vec<Token<'code>>, kind: &str) -> Result<Token<'code>, String> {
    let next_token = tokens.remove(0);

    if next_token.kind.name == kind {
        return Ok(next_token);
    }

    Err(next_token.kind.name.clone())
}
