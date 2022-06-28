//! Parser combinator, implemented in rust, for the chant programming language

//use anyhow::*;
use std::marker::PhantomData;

const OPERATOR_CHARS: &str = ":=+-/*^&%|<>!";
const SEPARATOR_CHARS: &str = ",.(){}[]";
const WHITESPACE_CHARS: &str = " \t\n";

use crate::lexer::*;

pub type ParseResult<'a, I, O> = Result<(I, O), ()>;

pub trait Parser<I, O>: Sized {
    /// The `I` returned should be a continuation of the input, where the items parsed have been removed.
    /// If `I` is an iter, the input can simply be returned at the end of the function.
    fn parse(&self, input: I) -> ParseResult<I, O>;

    fn map<U, F: Fn(O) -> U>(self, f: F) -> Map<Self, O, F> {
        Map(self, f, PhantomData)
    }

    fn to<U>(self, u: U) -> To<Self, O, U> {
        To(self, u, PhantomData)
    }
}

pub struct Map<A, O, F>(A, F, PhantomData<O>);

impl<Item, I: Iterator<Item = Item>, O, A: Parser<I, O>, U, F: Fn(O) -> U> Parser<I, U>
    for Map<A, O, F>
{
    fn parse(&self, input: I) -> ParseResult<I, U> {
        self.0.parse(input).map(|(i, o)| (i, self.1(o)))
    }
}

#[derive(Clone, Copy)]
pub struct To<A, O, U>(A, U, PhantomData<O>);

impl<Item, I: Iterator<Item = Item>, O, U: Clone, A: Parser<I, O>> Parser<I, U> for To<A, O, U> {
    fn parse(&self, input: I) -> ParseResult<I, U> {
        self.0.parse(input).map(|(i, _)| (i, self.1.clone()))
    }
}

pub struct TakeWhile<A>(A);

impl<Item, I: Iterator<Item = Item>, O, A: Parser<Item, O>> Parser<I, Vec<O>> for TakeWhile<A> {
    fn parse(&self, mut input: I) -> ParseResult<I, Vec<O>> {
        let mut values = Vec::new();
        for item in input {
            values.push(self.0.parse(item)?.1);
        }

        Ok((input, values))
    }
}

pub fn take_while<I, O, A: Parser<I, O>>(a: A) -> TakeWhile<A> {
    TakeWhile(a)
}

pub struct Char(char);

impl<I: Iterator<Item = char>> Parser<I, char> for Char {
    fn parse(&self, mut input: I) -> ParseResult<I, char> {
        match input.next() {
            Some(c) if c == self.0 => Ok((input, c)),
            _ => Err(()),
        }
    }
}

pub fn character(c: char) -> Char {
    Char(c)
}

/// Parser specialized for a specific use case. For example this could be a parser that only parses math expressions.
///
/// # Results
/// The parser should return a token, and the index for the remainding (unparsed) part of the input string.
///
/// For parsers that return `Token`, `Blank` should be returned when the parser (Self) is not
/// applicable to the input.
//pub trait Parser {
//    type Token;
//    fn parse(&self, input: &str) -> Result<(Self::Token, usize)>;
//
//    fn then<A: Parser>(self, other: A) -> Then<Self, A>
//    where
//        Self: Sized,
//    {
//        Then(self, other)
//    }
//
//    fn after_whitespace(self) -> EatPrecedingWhitespace<Self>
//    where
//        Self: Sized,
//    {
//        EatPrecedingWhitespace(self)
//    }
//
//    fn if_literal(self, literal: &str) -> IfLiteral<Self>
//    where
//        Self: Sized,
//    {
//        IfLiteral(self, literal.to_string())
//    }
//}

/// Parser for unsigned ints (list of digits)
pub struct NaturalNumber;

impl<'a> Parser<&'a str, Token<'a>> for NaturalNumber {
    fn parse(&self, i: &'a str) -> ParseResult<&'a str, Token<'a>> {
        let mut num = 0.;
        let mut rem = 0;
        for c in i.chars() {
            match format!("{c}").parse::<u8>() {
                std::result::Result::Ok(n) => num = num * 10. + n as f64,
                Err(_) => break,
            }
            rem += 1;
        }
        if rem == 0 {
            return Err(());
        }

        Ok((
            &i[rem..],
            Token {
                kind: TokenKind::Literal(Literal::Float(num)),
                len: rem,
            },
        ))
    }
}

/// Parser for any integer (list of digits, that might be pre-pended with '-')
pub struct Integer;

impl<'a> Parser<&'a str, Token<'a>> for Integer {
    fn parse(&self, i: &'a str) -> ParseResult<&'a str, Token<'a>> {
        if i.chars().nth(0) == Some('-') {
            let mut n = NaturalNumber.parse(&i[1..])?;
            if let TokenKind::Literal(Literal::Float(num)) = &mut n.0 {
                *n *= -1.;
            } else {
                return Ok((Token::Blank, 0));
            }
            n.1 += 1;
            Ok(n)
        } else {
            NaturalNumber.parse(i)
        }
    }
}

pub struct Float;

impl<'a> Parser<&'a str, Token<'a>> for Float {
    fn parse(&self, i: &str) -> ParseResult<&str, Token> {
        let mut num = Integer.parse(i)?;
        if i.chars().nth(num.1) != Some('.') {
            return Ok(num);
        }
        if num.0 == Token::Blank {
            num.0 = Token::Number(0.)
        }
        let mut decimalps = NaturalNumber.parse(&i[num.1 + 1..])?;
        if decimalps.0 == Token::Blank {
            return Ok(num);
        }
        *num.0.number() += *decimalps.0.number() / (10usize.pow(decimalps.1 as u32)) as f64
            * num.0.number().signum();
        num.1 += 1 + decimalps.1;
        Ok(num)
    }
}

pub struct Symbol;

impl<'a> Parser<&'a str, Token<'a>> for Symbol {
    fn parse(&self, i: &str) -> ParseResult<&str, Token> {
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

impl<'a> Parser<&'a str, Token<'a>> for Operator {
    fn parse(&self, i: &str) -> ParseResult<&str, Token> {
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

pub struct Separator;

impl<'a> Parser<&'a str, Token<'a>> for Separator {
    fn parse(&self, i: &str) -> ParseResult<&str, Token> {
        let c = i.chars().nth(0).unwrap();
        if !SEPARATOR_CHARS.contains(c) {
            Ok((Token::Blank, 0))
        } else {
            Ok((Token::Separator(c), 1))
        }
    }
}

pub struct Then<I, AO, BO, A: Parser<I, AO>, B: Parser<I, BO>>(
    A,
    B,
    PhantomData<I>,
    PhantomData<AO>,
    PhantomData<BO>,
);

impl<I, AO, BO, A: Parser<I, AO>, B: Parser<I, BO>> Parser<I, (AO, BO)> for Then<I, AO, BO, A, B> {
    fn parse(&self, i: I) -> ParseResult<I, (AO, BO)> {
        let a = self.0.parse(i)?;
        let b = self.1.parse(a.0)?;
        Ok((b.0, (a.1, b.1)))
    }
}

pub struct EatPrecedingWhitespace<'a, AO, A: Parser<&'a str, AO>>(A, &'a PhantomData<AO>);

impl<'a, AO, A: Parser<&'a str, AO>> Parser<&'a str, AO> for EatPrecedingWhitespace<'a, AO, A> {
    fn parse(&self, i: &str) -> ParseResult<&str, AO> {
        let mut rem = 0;
        for c in i.chars() {
            if !WHITESPACE_CHARS.contains(c) {
                break;
            }
            rem += 1;
        }

        let mut tmp = self.0.parse(&i[rem..])?;
        tmp.1 += rem;
        Ok(tmp)
    }
}

//pub struct IfLiteral<A: Parser>(A, String);
//
//impl<A: Parser> Parser for IfLiteral<A> {
//    type Token = Option<A::Token>;
//
//    fn parse(&self, i: &str) -> Result<(Self::Token, usize)> {
//        if i.len() < self.1.len() || i[0..self.1.len()] != self.1 {
//            Ok((None, 0))
//        } else {
//            let res = self.0.parse(&i[self.1.len()..])?;
//            Ok((Some(res.0), res.1 + self.1.len()))
//        }
//    }
//}

#[cfg(test)]
mod tests {
    use crate::parser::*;

    #[test]
    fn int() -> Result<(), ()> {
        assert_eq!(NaturalNumber.parse("123")?, (Token::Number(123.), 3));
        assert_eq!(NaturalNumber.parse("-123")?.0, Token::Blank);
        assert_eq!(Integer.parse("-123")?, (Token::Number(-123.), 4));
        assert_eq!(Integer.parse("123")?, (Token::Number(123.), 3));
        assert_eq!(Integer.parse("123abc")?, (Token::Number(123.), 3));
        Ok(())
    }

    #[test]
    fn symbol() -> Result<(), ()> {
        assert_eq!(
            Symbol.parse("_oki123")?,
            (Token::Symbol("_oki123".to_string()), 7)
        );
        assert_eq!(Symbol.parse("1_oki123")?.0, Token::Blank);
        Ok(())
    }

    #[test]
    fn op() -> Result<(), ()> {
        assert_eq!(
            Operator.parse("+=")?,
            (Token::Operator("+=".to_string()), 2)
        );
        Ok(())
    }

    #[test]
    fn num_then_symbol() -> Result<(), ()> {
        assert_eq!(
            Then(Integer, Symbol).parse("123abc")?,
            ((Token::Number(123.), Token::Symbol("abc".to_string())), 6)
        );

        Ok(())
    }

    #[test]
    fn symbol_then_num() -> Result<(), ()> {
        assert_eq!(
            Symbol.then(Integer.after_whitespace()).parse("abc 123")?,
            ((Token::Symbol("abc".to_string()), Token::Number(123.)), 7)
        );

        Ok(())
    }

    #[test]
    fn sep() -> Result<(), ()> {
        assert_eq!(Separator.parse("(())")?, (Token::Separator('('), 1));
        Ok(())
    }

    //#[test]
    //fn oneline_float_parser() -> Result<()> {
    //    let float = Integer.then(NaturalNumber.if_literal("."));

    //    assert_eq!(float.parse("123")?, ((Token::Number(123.), None), 3));
    //    assert_eq!(
    //        float.parse("-123.456")?,
    //        ((Token::Number(-123.), Some(Token::Number(456.))), 8)
    //    );
    //    Ok(())
    //}

    #[test]
    fn floats() -> Result<(), ()> {
        assert_eq!(Float.parse("-123.456")?, (Token::Number(-123.456), 8));
        assert_eq!(Float.parse("123")?, (Token::Number(123.), 3));
        assert_eq!(Float.parse("123.")?, (Token::Number(123.), 3));
        assert_eq!(Float.parse(".456")?, (Token::Number(0.456), 4));
        assert_eq!(Float.parse("-.456")?, (Token::Blank, 0));
        Ok(())
    }
}
