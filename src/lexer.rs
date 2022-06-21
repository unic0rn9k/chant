use std::str::Chars;

#[derive(Debug)]
pub enum Literal<'a> {
    Character(&'a str),
    String(&'a str),
    Number(&'a str),
}

#[derive(Debug)]
pub enum TokenKind<'a> {
    Whitespace,
    Literal(Literal<'a>),
}

impl<'a> TokenKind<'a> {
    pub fn into_token(self, len: usize) -> Token<'a> {
        Token::new(self, len)
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub len: usize,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind<'a>, len: usize) -> Self {
        Self { kind, len }
    }
}

#[derive(Debug)]
pub struct Tokens<'a>(Vec<Token<'a>>);

#[derive(Debug, Clone, Default)]
pub struct Location {
    pub filename: Option<String>,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    chars: Chars<'a>,
    location: Location,
    tokens: Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self::with_location(input, Location::default())
    }

    pub fn with_location(input: &'a str, location: Location) -> Self {
        Self {
            chars: input.chars(),
            location,
            tokens: Vec::default(),
        }
    }

    pub fn tokenize(mut self) -> Result<Tokens<'a>, ()> {
        let Self { chars, tokens, .. } = &mut self;
        while let Some(c) = chars.next() {
            println!("c {c}");
            let token = match c {
                c if is_whitespace(c) => Self::eat_whitespace(chars),
                _ => unreachable!(),
            };

            // TODO(Bech):
            // If a token error occurred we might be able to fix it by inserting the missing token.
            // - If we are unable to fix it we should not process any further tokens and stop!
            // - If we fixed it we should continue, but raise an error, so the rest of the compilation pipeline is aware an error was found.
            tokens.push(token?);
        }

        Ok(Tokens(self.tokens))
    }

    fn eat_whitespace(chars: &mut Chars) -> Result<Token<'a>, ()> {
        let len = chars.take_while(|c| is_whitespace(*c)).count();
        if len > 0 {
            return Ok(Token::new(TokenKind::Whitespace, len + 1));
        }

        Err(())
    }
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

pub fn tokenize(input: &str) -> Result<Tokens, ()> {
    let lexer = Lexer::new(input);
    lexer.tokenize()
}

#[cfg(test)]
mod tests {
    use super::tokenize;

    #[test]
    fn test_tokenize() {
        assert!(tokenize("    ").is_ok())
    }
}
