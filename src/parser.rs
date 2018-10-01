use super::node;
use super::token::Token;

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
