//! # TODO
//!
//! ## Strong mathematical numerical type system
//! - Natural numbers  (unsigned int)
//! - Integer          (signed int)
//! - Real numbers     (float)
//! - Complex
//! - Fast floats
//!
//! ## Misc
//! - Co-routines
//! - Manual Memory management
//! - Lifetimes
//! - Const-evaluation
//! - multiple dispatch (traits)

//#![feature(iter_advance_by)]
//#![feature(option_result_contains)]

use lexer::tokenize;

mod lexer;
mod parser;

fn main() {
    let input = " ";
    let tokens = tokenize(input);
    // println!("{tokens:?}");
}
