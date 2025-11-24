#[derive(Clone, Copy, Debug)]
pub struct Number {
    pub value: i32
}

#[derive(Clone, Debug)]
pub struct BinaryExpr {
    pub lhs: Box<AST>,
    pub op: String,
    pub rhs: Box<AST>
}

impl BinaryExpr {
    fn apply(self) -> AST {
        match self.op.as_str() {
            "+" => {
                let left = match self.lhs.eval() {
                    AST::Number(Number { value}) => value,
                    _ => 0
                };

                let right = match self.rhs.eval() {
                    AST::Number(Number { value}) => value,
                    _ => 0
                };

                AST::Number( Number { value: left + right })
            },
            "-" => {
                let left = match self.lhs.eval() {
                    AST::Number(Number { value}) => value,
                    _ => 0
                };

                let right = match self.rhs.eval() {
                    AST::Number(Number { value}) => value,
                    _ => 0
                };

                AST::Number( Number { value: left - right })
            },
            "/" => {
                let left = match self.lhs.eval() {
                    AST::Number(Number { value}) => value,
                    _ => 0
                };

                let right = match self.rhs.eval() {
                    AST::Number(Number { value}) => value,
                    _ => 0
                };

                AST::Number( Number { value: left / right })
            },
            "*" => {
                let left = match self.lhs.eval() {
                    AST::Number(Number { value}) => value,
                    _ => 0
                };

                let right = match self.rhs.eval() {
                    AST::Number(Number { value}) => value,
                    _ => 0
                };

                AST::Number( Number { value: left * right })
            },
            _ => panic!("Invalid operator!")
        }
    }
}

#[derive(Clone, Debug)]
pub enum AST {
    Number(Number),
    BinaryExpr(BinaryExpr)
}

impl AST {
    pub fn eval(self) -> AST {
        match self {
            AST::Number(_) => self,
            AST::BinaryExpr(b) => b.apply()
        }
    }

    pub fn into_i32(self) -> Result<i32, &'static str> {
        match self {
            AST::Number(Number { value}) => Ok(value),
            _ => Err("Failed to get AST as i32!")
        }
    }
}
