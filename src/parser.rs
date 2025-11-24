#![allow(dead_code)]

use std::collections::{VecDeque};

use crate::ast::*;

#[derive(Debug, Clone)]
enum Token {
    Number(i32),
    Operator(String)
}

pub struct Parser {
    expr: String,
    tokens: Vec<Token>,
    rpn: Vec<Token>,
    ast: AST,
    debug_enabled: bool
}

impl Parser {
    pub fn from(s: String) -> Self {
        Parser {
            expr: s,
            tokens: vec![],
            rpn: vec![],
            ast: AST::Number(Number { value: 0 }),
            debug_enabled: false
        }
    }

    pub fn enable_debug(&mut self) -> &mut Self {
        self.debug_enabled = true;

        self
    }

    pub fn disable_debug(&mut self) -> &mut Self {
        self.debug_enabled = false;

        self
    }

    pub fn parse_tokens(&mut self) -> &mut Self {
        let mut tokens: Vec<Token> = vec![];
        let mut i: usize = 0;

        while i < self.expr.len() {
            let c = self.expr.chars().nth(i).unwrap();

            if c.is_ascii_whitespace() {
                i += 1;
            } else if is_operator(c) {
                tokens.push(Token::Operator(c.to_string()));
                i += 1;
            } else if c.is_ascii_digit() {
                let mut buf = c.to_string();
                i += 1;

                let mut c = match self.expr.chars().nth(i) {
                    Some(c) => c,
                    None => 'A'
                };

                while i < self.expr.len() && c.is_ascii_digit() {
                    buf.push(c);
                    i += 1;

                    c = self.expr.chars().nth(i).unwrap();
                }

                tokens.push(Token::Number(buf.parse::<i32>().unwrap()));
                i += 1;
            } else {
                panic!("Encountered illegal character!");
            }
        }

        self.tokens = tokens;

        if self.debug_enabled {
            println!("Expression: {}\nTokens: {:?}", self.expr, self.tokens);
        }

        self
    }

    pub fn parse_rpn(&mut self) -> &mut Self {
        let mut output: Vec<Token> = vec![];
        let mut op_stack: Vec<Token> = vec![];

        for tok in self.tokens.iter() {
            match tok {
                Token::Number(_) => output.push(tok.clone()),
                Token::Operator(op) => {
                    if op_stack.is_empty() {
                        op_stack.push(tok.clone())
                    } else {
                        let op_prec = get_precedence(op);
                        let top_prec = get_precedence(match op_stack.last().unwrap() {
                            Token::Operator(op) => op,
                            _ => panic!("Failed to get last operator!")
                        });

                        if op_prec > top_prec {
                            op_stack.push(tok.clone())
                        } else {
                            while !op_stack.is_empty() {
                                output.push(op_stack.pop().unwrap());
                            }

                            op_stack.push(tok.clone())
                        }
                    }
                }
            }
        }

        while !op_stack.is_empty() {
            output.push(op_stack.pop().unwrap());
        }
        self.rpn = output;

        if self.debug_enabled {
            println!("RPN: {:?}", self.rpn);
        }

        self
    }

    pub fn parse_ast(&mut self) -> &Self {
        let mut output: VecDeque<AST> = VecDeque::new();
        let mut op_stack: Vec<Token> = vec![];

        for tok in self.rpn.iter() {
            match tok {
                Token::Number(n) => output.push_back(AST::Number(Number { value: n.clone() })),
                Token::Operator(op) => {
                    if op_stack.is_empty() {
                        op_stack.push(tok.clone());
                    } else {
                        let op_prec = get_precedence(op);
                        let top_prec = get_precedence(match op_stack.last().unwrap() {
                            Token::Operator(op) => op,
                            _ => panic!("Failed to get last operator!")
                        });

                        if op_prec > top_prec {
                            op_stack.push(tok.clone())
                        } else {
                            let left = output.pop_front().unwrap();
                            let right = output.pop_front().unwrap();

                            output.push_back(AST::BinaryExpr(BinaryExpr {
                                lhs: Box::new(left),
                                op: match op_stack.pop().unwrap() {
                                    Token::Operator(op) => op,
                                    _ => panic!("Failed to pop last expression")
                                },
                                rhs: Box::new(right)
                            }));

                            op_stack.push(tok.clone());
                        }
                    }
                }
            }
        }

        let left = output.pop_front().unwrap();
        let right = output.pop_front().unwrap();

        output.push_back(AST::BinaryExpr(BinaryExpr {
            lhs: Box::new(left),
            op: match op_stack.pop().unwrap() {
                Token::Operator(op) => op,
                _ => panic!("Failed to pop last expression")
            },
            rhs: Box::new(right)
        }));

        self.ast = output.iter().last().unwrap().clone();

        if self.debug_enabled {
            println!("AST: {:?}", self.ast);
        }

        self
    }

    pub fn collect(&self) -> AST {
        self.ast.clone()
    }
}

fn is_operator(c: char) -> bool {
    c == '+' || c == '-' || c == '/' || c == '*'
}

fn get_precedence(c: &String) -> i32 {
    if c == "+" || c == "-" {
        1
    } else if c == "*" || c == "/" {
        2
    } else {
        panic!("Failed to get precedence!")
    }
}
