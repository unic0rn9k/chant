//use std::iter::Peekable;
//use std::ops::Deref;
//use std::str::Chars;

use crate::parser::{character, take_while, Parser};

pub enum BinaryOp {
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `^`
    Caret,
    /// `&`
    And,
    /// `|`
    Or,
    /// `<<`
    Shl,
    /// `>>`
    Shr,
}

pub enum UnaryOp {
    /// `!`
    Bang,
    /// `~`
    Tilde,
    /// `?`
    Question,
}

pub enum Delim {
    /// `()`
    Paren,
    /// `{}`
    Brace,
    /// `[]`
    Bracket,
}

pub enum Literal {
    String(String),
    Integer(isize),
    Float(f64),
}

pub struct Ident<'a> {
    val: &'a str,
}

pub enum TokenKind<'a> {
    /// Any of the binary operators.
    BinaryOp(BinaryOp),
    /// Any of the binary operators preceeded by a `=`.
    BinaryOpEq(BinaryOp),
    /// Any of the unary operators.
    UnaryOp(UnaryOp),
    /// Any open delimiter.
    OpenDelim(Delim),
    /// Any close delimiter.
    CloseDelim(Delim),
    /// Any literal
    Literal(Literal),
    /// Any identifier
    Ident(Ident<'a>),
    /// `=`
    Eq,
    /// `:=`
    ColonEq,
    /// `<`
    Lt,
    /// `<=`
    Le,
    /// `==`
    EqEq,
    /// `>`
    Gt,
    /// `>=`
    Ge,
    /// `&&`
    AndAnd,
    /// `||`
    OrOr,
    /// `.`
    Dot,
    /// `..`
    DotDot,
    /// `...`
    DotDotDot,
    /// `,`
    Comma,
    /// `;`
    SemiColon,
    /// `:`
    Colon,

    /// A sequence of whitespace characters.
    /// We preserve whitespace to be able to reconstruct the input if an error happened.
    Whitespace,
}

pub struct Token<'a> {
    kind: TokenKind<'a>,
    len: usize,
}

pub struct Tokens<'a>(Vec<Token<'a>>);

pub fn tokenize(input: &str) -> Result<Tokens, ()> {
    let whitespace = take_while(character(' ')).parse(input.chars());
    println!("{whitespace:?}");
    Err(())
}
