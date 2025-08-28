use std::env;
use std::process;
use assembler::is_valid_file;
use assembler::tokenizer::Tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("USAGE: {} <file.s>", args[0]);
        process::exit(1);
    }
    let filename = &args[1];
    if !is_valid_file(filename) {
        eprintln!("Invalid filename or file doesn't exist");
    }
    let content = match std::fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };
    let mut tokenizer = Tokenizer::new(&content);
    match tokenizer.tokens() {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        },
        Err(err) => eprintln!("{}", err)
    }
}
