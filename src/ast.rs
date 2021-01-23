#[derive(Debug)]
pub enum Number {
    Constant(u64),
    Variable(char),
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

impl Expression {
    pub fn optimize(self: Self) -> Self {
        match self {
            Expression::Number(_) => self,

            Expression::Operation(op, left, right) => {
                match Expression::Operation(op, box left.optimize(), box right.optimize()) {
                    Expression::Operation(op, box Expression::Number(Number::Constant(x)), box Expression::Number(Number::Constant(y))) => {
                        match op {
                            Operation::Addition => Expression::Number(Number::Constant(x+y)),
                            Operation::Subtraction => Expression::Number(Number::Constant(x-y)),
                            Operation::Multiplication => Expression::Number(Number::Constant(x * y)),
                            Operation::Division => Expression::Operation(Operation::Division,
                                box Expression::Number(Number::Constant(x)),
                                box Expression::Number(Number::Constant(y))
                            ),
                            Operation::Exponentiation => Expression::Number(Number::Constant(x.pow(y as u32))),
                        }
                    },
                    expr => expr,
                }
            },
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
    pub fn optimize(self: Self) -> Self {
        Statement { left: self.left.optimize(), right: self.right.optimize() }
    }
}

