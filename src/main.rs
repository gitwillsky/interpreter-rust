use std::env;
use std::fs;
use std::io::Write;
use std::process::exit;

use log::error;
use lox::ast_printer::AstPrinter;
use lox::environment::Value;
use lox::interpreter::Interpreter;
use lox::lex::Literal;
use lox::lex::Tokenizer;
use lox::parser::Parser;
use lox::resolver::Resolver;

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
            let expression = parser.expression();
            match expression {
                Ok(expr) => {
                    let mut ast_printer = AstPrinter::new();
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
            let expression = parser.expression();
            match expression {
                Ok(expr) => {
                    let mut interpreter = Interpreter::new();
                    let result = interpreter.evaluate(&expr);
                    match result {
                        Ok(literal) => println!("{}", literal.to_string()),
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
        "run" => {
            let mut tokenizer = Tokenizer::new(file_contents);
            let (tokens, exit_code) = tokenizer.parse();
            if exit_code != 0 {
                exit(exit_code);
            }
            let mut parser = Parser::new(tokens);
            let statements = parser.parse();
            match statements {
                Ok(s) => {
                    let mut interpreter = Interpreter::new();
                    interpreter.define_native_function("clock".to_string(), |_| {
                        Ok(Value::Literal(Literal::Number(
                            std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs_f64(),
                        )))
                    });
                    let mut resolver = Resolver::new(&interpreter);
                    resolver.resolve_statements(&s)?;
                    match interpreter.interpret(&s) {
                        Ok(_) => (),
                        Err(e) => {
                            error!("{}", e);
                            exit(70);
                        }
                    }
                }
                Err(e) => {
                    error!("{}", e);
                    exit(65)
                }
            }
        }
        _ => {
            error!("Unknown command: {}", command);
            return;
        }
    }
}
