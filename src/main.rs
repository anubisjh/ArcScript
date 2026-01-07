mod lexer;
mod parser;
mod ast;
mod interpreter;

use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 && args[1] == "repl" {
        run_repl();
    } else {
        run_demo();
    }
}

fn run_demo() {
    let source = r#"
        // Simple ArcScript demo with a function
        func add(a, b): {
            return a + b;
        } end

        var result = add(2, 3);
    "#;

    let lexer = lexer::Lexer::new(source);
    let mut parser = parser::Parser::new(lexer);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(errors) => {
            eprintln!("Parse errors:");
            for err in errors {
                eprintln!("  {}:{}: {}", err.line, err.column, err.message);
            }
            return;
        }
    };

    let mut interp = interpreter::Interpreter::new();
    match interp.eval_program(&program) {
        Ok(_) => println!("ArcScript demo script executed (function add(2, 3) was called)."),
        Err(e) => eprintln!("Runtime error: {}", e),
    }
}

fn run_repl() {
    println!("ArcScript REPL v0.1.0");
    println!("Type 'exit' or press Ctrl+C to quit.\n");

    let mut interp = interpreter::Interpreter::new();
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        
        let input = input.trim();
        if input == "exit" || input.is_empty() {
            break;
        }
        
        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        
        match parser.parse_program() {
            Ok(program) => {
                match interp.eval_program(&program) {
                    Ok(_) => {},
                    Err(e) => eprintln!("Runtime error: {}", e),
                }
            }
            Err(errors) => {
                for err in errors {
                    eprintln!("Parse error at {}:{}: {}", err.line, err.column, err.message);
                }
            }
        }
    }
    
    println!("\nGoodbye!");
}
