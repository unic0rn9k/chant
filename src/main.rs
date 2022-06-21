//! # Strong mathematical numerical type system
//! - Natural numbers  (unsigned int)
//! - Integer          (signed int)
//! - Real numbers     (float)
//! - Complex
//! - Fast floats
//!
//! - Co-routines
//! - Manual Memory management
//! - Lifetimes
//! - Const-evaluation

mod lexer;
mod parser;

use lexer::tokenize;

fn main() {
    let input = "    d";
    let tokens = tokenize(input);
    println!("{tokens:?}");
}
