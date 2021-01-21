#[derive(Debug)]
pub enum Number {
    Constant(u64),
    Variable(String),
}

#[derive(Debug)]
pub enum Expression {
    Number(Number),
    Operation(Operation, Box<Expression>, Box<Expression>),
    Unary(Operation, Box<Expression>),
}

#[derive(Debug)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
}

#[derive(Debug)]
pub struct Statement {
    left: Expression,
    right: Expression,
}

pub fn parse_statement(math: String) -> Statement {
    let (left, math) = parse_expression(math.trim().chars().collect());

    if math.trim().chars().next().unwrap() != '=' {
        panic!("Exprected '='");
    }

    let (right, math) = parse_expression(
        math.trim()
            .chars()
            .skip(1)
            .collect::<String>()
            .trim()
            .chars()
            .collect(),
    );

    if math.trim().len() != 0 {
        panic!("Expected end of string");
    }

    Statement { left, right }
}

fn get_first(s: &String) -> char {
    s.trim().chars().next().unwrap()
}

fn get_rest(s: &String) -> String {
    s.trim()
        .chars()
        .skip(1)
        .collect::<String>()
        .trim()
        .chars()
        .collect()
}

fn pull(s: &String) -> (char, String) {
    (get_first(s), get_rest(s))
}

pub fn parse_expression(math: String) -> (Expression, String) {
    parse_addition(math)
}

pub fn parse_addition(math: String) -> (Expression, String) {
    let (left, math) = parse_multiplication(math);

    if math.len() > 0 {
        let (first, rest) = pull(&math);

        match first {
            '+' | '-' => {
                let (right, math) = parse_addition(rest);
                (
                    Expression::Operation(
                        match first {
                            '+' => Operation::Addition,
                            '-' => Operation::Subtraction,
                            _ => panic!("Excuse me WTF"),
                        },
                        box left,
                        box right,
                    ),
                    math,
                )
            }
            _ => (left, math),
        }
    } else {
        (left, math)
    }
}

pub fn parse_multiplication(math: String) -> (Expression, String) {
    let (left, math) = parse_exponentiation(math);

    if math.len() > 0 {
        let (first, rest) = pull(&math);

        match first {
            '*' | '/' => {
                let (right, math) = parse_multiplication(rest);
                (
                    Expression::Operation(
                        match first {
                            '*' => Operation::Multiplication,
                            '/' => Operation::Division,
                            _ => panic!("Excuse me WTF"),
                        },
                        box left,
                        box right,
                    ),
                    math,
                )
            }
            _ => (left, math),
        }
    } else {
        (left, math)
    }
}

pub fn parse_exponentiation(math: String) -> (Expression, String) {
    let (left, math) = parse_unary(math);

    if math.len() > 0 {
        let (first, rest) = pull(&math);

        match first {
            '^' => {
                let (right, math) = parse_exponentiation(rest);
                (
                    Expression::Operation(
                        match first {
                            '^' => Operation::Exponentiation,
                            _ => panic!("Excuse me WTF"),
                        },
                        box left,
                        box right,
                    ),
                    math,
                )
            }
            _ => (left, math),
        }
    } else {
        (left, math)
    }
}

pub fn parse_unary(math: String) -> (Expression, String) {
    if math.len() > 0 {
        let (first, rest) = pull(&math);

        match first {
            '+' | '-' => {
                let (unary, math) = parse_unary(rest);
                (
                    Expression::Unary(
                        match first {
                            '+' => Operation::Addition,
                            '-' => Operation::Subtraction,
                            _ => panic!("WTF"),
                        },
                        box unary,
                    ),
                    math,
                )
            }
            _ => parse_low(math),
        }
    } else {
        panic!("Error, need more math")
    }
}

pub fn parse_low(math: String) -> (Expression, String) {
    let (first, rest) = pull(&math);

    match first {
        x if x.is_digit(10) => parse_num(math),
        x if x.is_alphabetic() => (Expression::Number(Number::Variable(x.to_string())), rest),
        '(' => {
            let (expr, math) = parse_expression(rest);
            let (first, math) = pull(&math);
            if first != ')' {
                panic!("Error, expected ')'");
            }
            (expr, math)
        }
        _ => panic!("Error"),
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
