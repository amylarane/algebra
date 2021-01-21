#![feature(box_syntax)]
#[macro_use]
extern crate text_io;

use std::io::{self, Write};

mod parse;

fn main() {
    print!("Input Statement: ");
    io::stdout().flush().unwrap();

    let s: String;
    scan!("{}\n", s);

    println!("{:?}", parse::parse_statement(s));
}
