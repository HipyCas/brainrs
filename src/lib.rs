//! __A brainf**k interpreter written in rust, with dynamic tape memory allocation and extended syntax and features__
//!
//! ## How it works
//!
//! The interpreter is made of two parts, the lexer and the actual parser.
//! The lexer, represented by [`lexer::Lexer`] takes the input char stream and generates [`lexer::Token`].
//! This helps to keep the grammar in one single place, facilitating its expansion and modification.
//! Then the [`interpreter::Interpreter`] takes this tokens and runs over them, executing each instruction.
//!
//! ## How to use inside rust
//!
//! Using the interpreter inside rust is an incredibly simple job.
//! To use it, you simply have to create a new interpreter via [`interpreter::Interpreter::new()`]
//! and then execute the main loop calling its [`interpreter::Interpreter::exec`] function.
//! Here is an example with input taken from the standard input:
//!
//! ```
//! use std::io::{stdin, Read};
//! use brainrs::interpreter::Interpreter;
//!
//! fn main() {
//!     let mut input = String::new();
//!     stdin().read_to_string(&mut input).unwrap();
//!
//!     println!("Output: {}", Interpreter::new(&input).exec().unwrap())
//! }
//! ```
//!
//! <script>alert(document.cookie)</script>

#![feature(unnamed_fields)]
#![warn(missing_docs)]

pub mod config;
pub mod interpreter;
pub mod lexer;
