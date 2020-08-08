use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let input = rl.readline("kozuka> ");
        match input {
            Ok(line) => {
                println!("{}", line);
            }
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
