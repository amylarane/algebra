#![feature(box_syntax)]
#![feature(box_patterns)]

#[macro_use]
extern crate text_io;

use std::io::{self, Write};

mod ast;
mod parse;
mod utils;

fn main() {
    print!("Input Statement: ");
    io::stdout().flush().unwrap();

    let s: String;
    scan!("{}\n", s);

    println!("{}", parse::parse_statement(s).optimize().to_string());
}
