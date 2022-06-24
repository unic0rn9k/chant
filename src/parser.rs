use std::marker::PhantomData;

pub type ParseResult<'a, Iter, O> = Result<(Iter, O), ()>;

pub trait Parser<I, O>: Sized {
    fn parse<'a, Iter: Iterator<Item = I>>(&'a self, input: Iter) -> ParseResult<Iter, O>;

    fn map<U, F: Fn(O) -> U>(self, f: F) -> Map<Self, O, F> {
        Map(self, f, PhantomData)
    }

    fn to<U>(self, u: U) -> To<Self, O, U> {
        To(self, u, PhantomData)
    }
}

pub struct Map<A, O, F>(A, F, PhantomData<O>);

impl<I, O, A: Parser<I, O>, U, F: Fn(O) -> U> Parser<I, U> for Map<A, O, F> {
    fn parse<'a, Iter: Iterator<Item = I>>(&'a self, input: Iter) -> ParseResult<Iter, U> {
        self.0.parse(input).map(|(i, o)| (i, self.1(o)))
    }
}

#[derive(Clone, Copy)]
pub struct To<A, O, U>(A, U, PhantomData<O>);

impl<I, O, U: Clone, A: Parser<I, O>> Parser<I, U> for To<A, O, U> {
    fn parse<'a, Iter: Iterator<Item = I>>(&'a self, input: Iter) -> ParseResult<Iter, U> {
        self.0.parse(input).map(|(i, _)| (i, self.1.clone()))
    }
}

pub struct TakeWhile<A>(A);

impl<I, O, A: Parser<I, O>> Parser<I, Vec<O>> for TakeWhile<A> {
    fn parse<'a, Iter: Iterator<Item = I>>(&'a self, mut input: Iter) -> ParseResult<Iter, Vec<O>> {
        let mut values = Vec::new();
        while let Some(input) = input.next() {
            match self.0.parse([input].into_iter()) {
                Ok((_, o)) => values.push(o),
                _ => todo!(),
            }
        }

        Ok((input, values))
    }
}

pub fn take_while<I, O, A: Parser<I, O>>(a: A) -> TakeWhile<A> {
    TakeWhile(a)
}

pub struct Char(char);

impl<'b> Parser<char, ()> for Char {
    fn parse<Iter: Iterator<Item = char>>(&self, mut input: Iter) -> ParseResult<Iter, ()> {
        match input.next() {
            Some(c) if c == self.0 => Ok((input, ())),
            _ => Err(()),
        }
    }
}

pub fn character(c: char) -> Char {
    Char(c)
}
// #[derive(Clone, Copy)]
// pub struct Literal<I>(I);

// impl<'b> Parser<&'b str, ()> for Literal {
//     fn parse<'a>(&self, input: &'a &'b str) -> ParseResult<&'a str, ()> {
//         match input.starts_with(self.0) {
//             true => Ok((input[self.0.len()..], ())),
//             _ => Err(()),
//         }
//     }
// }

// pub fn literal(s: &'static str) -> Literal {
//     Literal(s)
// }
