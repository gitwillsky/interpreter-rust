use std::env;
use std::fs;
use std::io::Write;
use std::process::exit;

use codecrafters_interpreter::ast_printer::AstPrinter;
use codecrafters_interpreter::interpreter::Interpreter;
use codecrafters_interpreter::lex::Tokenizer;
use codecrafters_interpreter::parser::Parser;
use log::error;

fn main() {
    env_logger::builder()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        error!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        error!("Failed to read file {}", filename);
        String::new()
    });

    match command.as_str() {
        "tokenize" => {
            let mut tokenizer = Tokenizer::new(file_contents);
            let (tokens, exit_code) = tokenizer.parse();
            tokens.iter().for_each(|token| println!("{}", token));
            exit(exit_code);
        }
        "parse" => {
            let mut tokenizer = Tokenizer::new(file_contents);
            let (tokens, exit_code) = tokenizer.parse();
            if exit_code != 0 {
                exit(exit_code);
            }
            let mut parser = Parser::new(tokens);
            let expression = parser.parse();
            match expression {
                Ok(expr) => {
                    let ast_printer = AstPrinter::new();
                    println!("{}", ast_printer.print(&expr));
                }
                Err(e) => {
                    error!("{}", e);
                    exit(65);
                }
            }
        }
        "evaluate" => {
            let mut tokenizer = Tokenizer::new(file_contents);
            let (tokens, exit_code) = tokenizer.parse();
            if exit_code != 0 {
                exit(exit_code);
            }
            let mut parser = Parser::new(tokens);
            let expression = parser.parse();
            match expression {
                Ok(expr) => {
                    let interpreter = Interpreter::new();
                    let result = interpreter.interpret(&expr);
                    match result {
                        Ok(result) => println!("{}", result),
                        Err(e) => {
                            error!("{}", e);
                            exit(70);
                        }
                    }
                }
                Err(e) => {
                    error!("{}", e);
                    exit(65);
                }
            }
        }
        _ => {
            error!("Unknown command: {}", command);
            return;
        }
    }
}
