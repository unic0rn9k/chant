//! Parser combinator, implemented in rust, for the chant programming language

use std::marker::PhantomData;

use anyhow::*;

const OPERATOR_CHARS: &str = ":=+-/*^&%|<>!";
const SEPARATOR_CHARS: &str = ",.(){}[]";

/// A basic token type.
///
/// The Blank token should always be ignored.
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Token {
    Symbol(String),
    Number(isize),
    String(String),
    Operator(String),
    Separator(char),
    Blank,
    Then(Box<Token>, Box<Token>),
}

impl Token {
    fn number(&mut self) -> &mut isize {
        if let Token::Number(n) = self {
            n
        } else {
            panic!("{self:?} is not a number")
        }
    }
}

/// Parser specialized for a specific use case. For example this could be a parser that only parses math expressions.
///
/// # Results
/// The parser should return a token, and the index for the remainding (unparsed) part of the input string.
pub trait Combinator {
    fn parse(input: &str) -> Result<(Token, usize)>;
}

/// Parser for unsigned ints (list of digits)
pub struct NaturalNumber;

impl Combinator for NaturalNumber {
    fn parse(i: &str) -> Result<(Token, usize)> {
        let mut num = 0;
        let mut rem = 0;
        for c in i.chars() {
            match format!("{c}").parse::<usize>() {
                std::result::Result::Ok(n) => num = num * 10 + n as isize,
                Err(_) => break,
            }
            rem += 1;
        }
        if rem == 0 {
            return Ok((Token::Blank, 0));
        }
        Ok((Token::Number(num), rem))
    }
}

/// Parser for any integer (list of digits, that might be pre-pended with '-')
pub struct Integer;

impl Combinator for Integer {
    fn parse(i: &str) -> Result<(Token, usize)> {
        if i.chars().nth(0) == Some('-') {
            let mut n = NaturalNumber::parse(&i[1..])?;
            *n.0.number() *= -1;
            n.1 += 1;
            Ok(n)
        } else {
            NaturalNumber::parse(i)
        }
    }
}

pub struct Symbol;

impl Combinator for Symbol {
    fn parse(i: &str) -> Result<(Token, usize)> {
        let mut buffer = vec![];
        let mut i = i.chars();

        // check that first charecter is alphabetical og '_'
        let fc = i.next().unwrap() as u8;
        if fc == 95 || (fc > 64 && fc < 91) || (fc > 96 && fc < 123) {
            buffer.push(fc as char)
        } else {
            return Ok((Token::Blank, 0));
        }

        // all other charecters can also be numbers...
        let mut rem = 1;
        for c in i {
            let c = c as u8;
            if c == 95 || (c > 64 && c < 91) || (c > 96 && c < 123) || (c > 47 && c < 58) {
                buffer.push(c as char)
            } else {
                break;
            }
            rem += 1;
        }

        Ok((Token::Symbol(buffer.iter().collect()), rem))
    }
}

pub struct Operator;

impl Combinator for Operator {
    fn parse(i: &str) -> Result<(Token, usize)> {
        let mut rem = 0;

        for c in i.chars() {
            if !OPERATOR_CHARS.contains(c) {
                break;
            }
            rem += 1
        }

        if rem == 0 {
            return Ok((Token::Blank, 0));
        }

        Ok((Token::Operator((&i[0..rem]).to_string()), rem))
    }
}

pub struct Then<A: Combinator, B: Combinator>(PhantomData<A>, PhantomData<B>);

impl<A: Combinator, B: Combinator> Combinator for Then<A, B> {
    fn parse(i: &str) -> Result<(Token, usize)> {
        let a = A::parse(i)?;
        let b = B::parse(&i[a.1..])?;
        Ok((Token::Then(Box::new(a.0), Box::new(b.0)), a.1 + b.1))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::*;

    #[test]
    fn int() -> Result<()> {
        assert_eq!(NaturalNumber::parse("123")?, (Token::Number(123), 3));
        assert_eq!(NaturalNumber::parse("-123")?.0, Token::Blank);
        assert_eq!(Integer::parse("-123")?, (Token::Number(-123), 4));
        assert_eq!(Integer::parse("123")?, (Token::Number(123), 3));
        assert_eq!(Integer::parse("123abc")?, (Token::Number(123), 3));
        Ok(())
    }

    #[test]
    fn symbol() -> Result<()> {
        assert_eq!(
            Symbol::parse("_oki123")?,
            (Token::Symbol("_oki123".to_string()), 7)
        );
        assert_eq!(Symbol::parse("1_oki123")?.0, Token::Blank);
        Ok(())
    }

    #[test]
    fn op() -> Result<()> {
        assert_eq!(
            Operator::parse("+=")?,
            (Token::Operator("+=".to_string()), 2)
        );
        Ok(())
    }

    #[test]
    fn symbol_then_num() -> Result<()> {
        assert_eq!(
            Then::<Integer, Symbol>::parse("123abc")?,
            (
                Token::Then(
                    Box::new(Token::Number(123)),
                    Box::new(Token::Symbol("abc".to_string())),
                ),
                6
            )
        );

        Ok(())
    }
}
