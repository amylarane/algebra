#[derive(Debug)]
pub enum Number {
    Constant(u64),
    Variable(String),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
}

#[derive(Debug)]
pub enum Expression {
    Number(Number),
    Operation(Operation, Box<Expression>, Box<Expression>),
    Unary(Operation, Box<Expression>),
}

#[derive(Debug)]
pub struct Statement {
    left: Expression,
    right: Expression,
}

fn get_first(s: &String) -> Option<char> {
    s.trim().chars().next()
}

fn get_rest(s: &String) -> Option<String> {
    match s.len() {
        0 => None,
        _ => Some(
            s.trim()
                .chars()
                .skip(1)
                .collect::<String>()
                .trim()
                .chars()
                .collect(),
        ),
    }
}

fn pull(s: &String) -> (Option<char>, Option<String>) {
    (get_first(s), get_rest(s))
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
        panic!("Expected end of string")
    }

    Statement { left, right }
}

use std::collections::HashMap;
pub struct OpTable {
    operations: HashMap<char, Operation>,
    next_level: Option<Box<OpTable>>,
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
            operations: [('+', Operation::Addition), ('-', Operation::Subtraction)]
                .iter()
                .cloned()
                .collect(),
            next_level: Some(box OpTable {
                operations: [('*', Operation::Multiplication), ('/', Operation::Division)]
                    .iter()
                    .cloned()
                    .collect(),
                next_level: Some(box OpTable {
                    operations: [('^', Operation::Exponentiation)].iter().cloned().collect(),
                    next_level: None,
                }),
            }),
        },
    )
}

pub fn parse_unary(math: String) -> (Expression, String) {
    match pull(&math) {
        (Some(first), Some(rest)) if first == '+' || first == '-' => {
            let (operation, math) = parse_unary(rest);
            (
                Expression::Unary(
                    match first {
                        '+' => Operation::Addition,
                        '-' => Operation::Subtraction,
                        _ => panic!("Unexpected operator you broke something"),
                    },
                    box operation,
                ),
                math,
            )
        }
        (Some(_), Some(_)) => parse_low(math),
        _ => panic!("Error, expected rest of expression got end of string"),
    }
}

pub fn parse_low(math: String) -> (Expression, String) {
    match pull(&math) {
        (Some('('), Some(rest)) => {
            let (expr, math) = parse_expression(rest);
            match pull(&math) {
                (Some(')'), Some(rest)) => (expr, rest),
                (Some(first), _) => panic!("Expected ')' got {}", first),
                _ => panic!("Expected ')' got end of string"),
            }
        }
        (Some(first), _) if first.is_digit(10) => parse_num(math),
        (Some(first), rest) if first.is_alphabetic() => (
            Expression::Number(Number::Variable(first.to_string())),
            match rest {
                Some(string) => string,
                None => "".to_string(),
            },
        ),
        (Some(first), Some(_)) => panic!("Unexpected character {}", first),
        (_, _) => panic!("Unexpected end of string"),
    }
}

pub fn parse_num(math: String) -> (Expression, String) {
    let mut value = 0;
    let mut index = 0;

    while index < math.len() && math.chars().nth(index).unwrap().is_digit(10) {
        value *= 10;
        value += math.chars().nth(index).unwrap() as u64 - '0' as u64;
        index += 1;
    }

    (
        Expression::Number(Number::Constant(value)),
        math.chars()
            .skip(index)
            .collect::<String>()
            .trim()
            .chars()
            .collect(),
    )
}
