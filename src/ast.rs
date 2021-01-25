#[derive(Debug)]
pub enum Number {
    Constant(u64),
    Variable(char),
}

impl Number {
    pub fn to_string(self: Self) -> String {
        match self {
            Number::Constant(value) => value.to_string(),
            Number::Variable(value) => value.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
}

impl Operation {
    pub fn to_string(self: Self) -> String {
        match self {
            Operation::Addition => "+".to_string(),
            Operation::Subtraction => "-".to_string(),
            Operation::Multiplication => "*".to_string(),
            Operation::Division => "/".to_string(),
            Operation::Exponentiation => "^".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Number(Number),
    Operation(Operation, Box<Expression>, Box<Expression>),
    Unary(Operation, Box<Expression>),
}

impl Expression {
    pub fn to_string(self: Self) -> String {
        match self {
            Expression::Number(number) => number.to_string(),
            Expression::Operation(op, box left, box right) => format!("{} {} {}", left.to_string(), op.to_string(), right.to_string()),
            Expression::Unary(op, box expr) => format!("{}{}", op.to_string(), expr.to_string()),
        }
    }

    pub fn optimize(self: Self) -> Self {
        match self {
            Expression::Number(_) => self,

            Expression::Operation(op, left, right) => {
                match Expression::Operation(op, box left.optimize(), box right.optimize()) {
                    Expression::Operation(
                        op,
                        box Expression::Number(Number::Constant(x)),
                        box Expression::Number(Number::Constant(y)),
                    ) => match op {
                        Operation::Addition => Expression::Number(Number::Constant(x + y)),
                        Operation::Subtraction => match x as i64 - y as i64 {
                            val if val < 0 => Expression::Unary(Operation::Subtraction, box Expression::Number(Number::Constant((-val) as u64))),
                            val => Expression::Number(Number::Constant(val as u64)),
                        },
                        Operation::Multiplication => Expression::Number(Number::Constant(x * y)),
                        Operation::Division => Expression::Operation(
                            Operation::Division,
                            box Expression::Number(Number::Constant(x)),
                            box Expression::Number(Number::Constant(y)),
                        ),
                        Operation::Exponentiation => {
                            Expression::Number(Number::Constant(x.pow(y as u32)))
                        }
                    },
                    expr => expr,
                }
            }
            Expression::Unary(op, expr) => Expression::Unary(op, box expr.optimize()),
        }
    }
}

#[derive(Debug)]
pub struct Statement {
    pub left: Expression,
    pub right: Expression,
}

impl Statement {
    pub fn to_string(self: Self) -> String {
        format!("{} = {}", self.left.to_string(), self.right.to_string())
    }

    pub fn optimize(self: Self) -> Self {
        Statement {
            left: self.left.optimize(),
            right: self.right.optimize(),
        }
    }
}
