use std::env;
use std::fs;
use std::io::Write;
use std::process::exit;

use codecrafters_interpreter::ast_printer::AstPrinter;
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

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                error!("Failed to read file {}", filename);
                String::new()
            });

            let mut tokenizer = Tokenizer::new(file_contents);
            let (tokens, exit_code) = tokenizer.parse();
            tokens.iter().for_each(|token| println!("{}", token));
            exit(exit_code);
        }
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                error!("Failed to read file {}", filename);
                String::new()
            });
            let mut tokenizer = Tokenizer::new(file_contents);
            let (tokens, exit_code) = tokenizer.parse();
            let mut parser = Parser::new(tokens);
            let expression = parser.parse();
            let ast_printer = AstPrinter::new();
            println!("{}", ast_printer.print(&expression.unwrap()));
            exit(exit_code);
        }
        _ => {
            error!("Unknown command: {}", command);
            return;
        }
    }
}
