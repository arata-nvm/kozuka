use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::fmt;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::{Pair, Pairs};
use pest::Parser;

#[derive(Parser)]
#[grammar = "lisp.pest"]
struct LispParser;

enum LispError {
    DivZero,
    BadOp,
    BadNum,
}

impl fmt::Display for LispError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LispError::DivZero => write!(f, "Division By Zero"),
            LispError::BadOp => write!(f, "Invalid Operator"),
            LispError::BadNum => write!(f, "Invalid Number"),
        }
    }
}

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let input = rl.readline("kozuka> ");
        match input {
            Ok(line) => exec(&line),
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Failed to read line: {}", err);
                break;
            }
        }
    }
}

fn exec(line: &str) {
    match LispParser::parse(Rule::expr, &line) {
        Ok(mut pairs) => match eval(&mut pairs) {
            Ok(value) => println!("{}", value),
            Err(err) => println!("Error: {}", err),
        },
        Err(err) => {
            println!("Failed to parse program: {}", err);
            return;
        }
    }
}

fn eval(pairs: &mut Pairs<Rule>) -> Result<i32, LispError> {
    eval_expr(pairs.next().unwrap())
}

fn eval_expr(pair: Pair<Rule>) -> Result<i32, LispError> {
    let mut iter = pair.into_inner();
    let first_pair = iter.next().unwrap();
    match first_pair.as_rule() {
        Rule::number => match first_pair.as_str().parse::<i32>() {
            Ok(num) => Ok(num),
            Err(_) => Err(LispError::BadNum),
        },
        Rule::operator => {
            let op = first_pair.as_str();
            let mut x = eval_expr(iter.next().unwrap())?;

            // unary minus operator
            if op == "-" && iter.peek() == None {
                return Ok(-x);
            }

            loop {
                match iter.next() {
                    Some(expr) => {
                        let y = eval_expr(expr)?;
                        x = eval_op(op, x, y)?;
                    }
                    None => break,
                }
            }
            Ok(x)
        }
        _ => unreachable!(),
    }
}

fn eval_op(op: &str, lhs: i32, rhs: i32) -> Result<i32, LispError> {
    match op {
        "+" => Ok(lhs + rhs),
        "-" => Ok(lhs - rhs),
        "*" => Ok(lhs * rhs),
        "/" => {
            if rhs == 0 {
                Err(LispError::DivZero)
            } else {
                Ok(lhs / rhs)
            }
        }
        "%" => Ok(lhs % rhs),
        "^" => Ok(lhs.pow(rhs as u32)), // TODO
        "min" => Ok(lhs.min(rhs)),
        "max" => Ok(lhs.max(rhs)),
        _ => Err(LispError::BadOp),
    }
}
