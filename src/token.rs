use regex::Regex;

/// TokenKind
pub struct TokenKind{
    pub name: String,
    pub re: Regex,
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
    pub kind: &'code TokenKind,
    pub value: &'code str,
}

impl<'code> Token<'code> {
    pub fn new(kind: &'code TokenKind, value: &'code str) -> Token<'code> {
        Token { kind, value }
    }
}

