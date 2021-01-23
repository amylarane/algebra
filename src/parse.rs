use crate::utils::*;
use crate::ast::*;

use std::collections::HashMap;
pub struct OpTable {
    operations: HashMap<char, Operation>,
    next_level: Option<Box<OpTable>>,
}

pub fn parse_statement(math: String) -> Statement {
    let (left, math) = parse_expression(math);

    let (right, math) = match pull(&math) {
        (Some(first), Some(rest)) => match first {
            '=' => parse_expression(rest),
            c => panic!("Expected '=' found {}", c),
        },
        (Some(first), None) => match first {
            '=' => panic!("Expected right side of equation, got end of string"),
            _ => panic!("Got unexpected character"),
        },
        _ => panic!("unexpected end of string"),
    };

    if math.len() != 0 {
        println!("{}", math);
        panic!("Expected end of string")
    }

    Statement { left, right }
}

pub fn parse_binary(math: String, operations: &OpTable) -> (Expression, String) {
    match operations.next_level {
        Some(ref table) => {
            let (left, math) = parse_binary(math, table);

            match pull(&math) {
                (Some(first), Some(rest)) if operations.operations.contains_key(&first) => {
                    let (right, math) = parse_binary(rest, operations);
                    (
                        Expression::Operation(
                            operations.operations[&first].clone(),
                            box left,
                            box right,
                        ),
                        math,
                    )
                }
                _ => (left, math),
            }
        }
        None => parse_unary(math),
    }
}

pub fn parse_expression(math: String) -> (Expression, String) {
    parse_binary(
        math,
        &OpTable {
            operations: array_to_hash_map(&[('+', Operation::Addition), ('-', Operation::Subtraction)]),
            next_level: Some(box OpTable {
                operations: array_to_hash_map(&[
                    ('*', Operation::Multiplication),
                    ('/', Operation::Division),
                ]),
                next_level: Some(box OpTable {
                    operations: array_to_hash_map(&[('^', Operation::Exponentiation)]),
                    next_level: Some(box OpTable {
                        operations: array_to_hash_map(&[]),
                        next_level: None
                    })
                }),
            }),
        },
    )
}

pub fn parse_unary(math: String) -> (Expression, String) {
    let ops = &OpTable {
        operations: array_to_hash_map(&[('+', Operation::Addition), ('-', Operation::Subtraction)]),
        next_level: None,
    };

    match pull(&math) {
        (Some(first), Some(rest)) if ops.operations.contains_key(&first) => {
            let (operation, math) = parse_unary(rest);
            (
                Expression::Unary(ops.operations[&first].clone(), box operation),
                math,
            )
        }
        (Some(_), _) => parse_low(math),
        (None, _) => panic!("Expected rest of expression, got end of string"),
    }
}

pub fn parse_low(math: String) -> (Expression, String) {
    match pull(&math) {
        (Some('('), _) => parse_paren(math),
        (Some(first), _) if first.is_digit(10) => parse_num(math),
        (Some(first), _) if first.is_alphabetic() => parse_var(math),
        (Some(first), _) => panic!("Unexpected character {}", first),
        (None, _) => panic!("Unexpected end of string"),
    }
}

pub fn parse_paren(math: String) -> (Expression, String) {
    match pull(&math) {
        (Some('('), Some(rest)) => {
            let (expr, math) = parse_expression(rest);
            match pull(&math) {
                (Some(')'), rest) => (expr, opt_to_string(rest)),
                (Some(first), _) => panic!("Expected ')' got {}", first),
                (None, _) => panic!("Expected ')'  got end of string"),
            }
        }
        _ => panic!("Unexpected character or end of string"),
    }
}

pub fn parse_var(math: String) -> (Expression, String) {
    match pull(&math) {
        (Some(first), rest) if first.is_alphabetic() => (
            Expression::Number(Number::Variable(first)),
            opt_to_string(rest),
        ),
        (_, _) => panic!("Unexpected character"),
    }
}

pub fn parse_num(math: String) -> (Expression, String) {
    let mut value = 0;
    let mut index = 0;

    while index < math.len() && get_at(&math, index).unwrap().is_digit(10) {
        value *= 10;
        value += get_at(&math, index).unwrap() as u64 - '0' as u64;
        index += 1;
    }

    (
        Expression::Number(Number::Constant(value)),
        get_rest(&math, index),
    )
}
