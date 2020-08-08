use rustyline::error::ReadlineError;
use rustyline::Editor;

extern crate pest;
#[macro_use]
extern crate pest_derive;

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
    println!("{}", line);
    match LispParser::parse(Rule::program, &line) {
        Ok(pairs) => {
            println!("{:?}", pairs);
        }
        Err(err) => {
            println!("Failed to parse program: {}", err);
            return;
        }
    }
}
