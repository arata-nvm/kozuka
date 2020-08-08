use rustyline::error::ReadlineError;
use rustyline::Editor;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::{Pair, Pairs};
use pest::Parser;

#[derive(Parser)]
#[grammar = "lisp.pest"]
struct LispParser;

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
        Ok(mut pairs) => eval(&mut pairs),
        Err(err) => {
            println!("Failed to parse program: {}", err);
            return;
        }
    }
}

fn eval(pairs: &mut Pairs<Rule>) {
    let value = eval_expr(pairs.next().unwrap());
    println!("{}", value);
}

fn eval_expr(pair: Pair<Rule>) -> i32 {
    let mut iter = pair.into_inner();
    let first_pair = iter.next().unwrap();
    match first_pair.as_rule() {
        Rule::number => first_pair.as_str().parse::<i32>().unwrap(),
        Rule::operator => {
            let op = first_pair.as_str();
            let mut x = eval_expr(iter.next().unwrap());

            loop {
                match iter.next() {
                    Some(expr) => x = eval_op(op, x, eval_expr(expr)),
                    None => break,
                }
            }
            x
        }
        _ => unreachable!(),
    }
}

fn eval_op(op: &str, lhs: i32, rhs: i32) -> i32 {
    match op {
        "+" => lhs + rhs,
        "-" => lhs - rhs,
        "*" => lhs * rhs,
        "/" => lhs / rhs,
        "%" => lhs % rhs,
        _ => unreachable!(),
    }
}
