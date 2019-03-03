use super::token::{Token, TokenKind};

pub fn tokenize<'code>(
    code: &'code str,
    tokens: &mut Vec<Token<'code>>,
    token_kinds: &'code [TokenKind]
) {
    if !code.is_empty() {
        let (more_code, token_option) = get_next_token(code, token_kinds);

        if let Some(token) = token_option {
            tokens.push(token);
        }

        tokenize(more_code, tokens, token_kinds)
    }
}

fn get_next_token<'code>(
    code: &'code str,
    token_kinds: &'code [TokenKind],
) -> (&'code str, Option<Token<'code>>) {
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
