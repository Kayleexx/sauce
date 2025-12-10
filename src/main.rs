use chumsky::Parser as _;
use sauce::lexer::{Lexer, SpannedToken};
use sauce::parser::{parser_statement};
use chumsky::IterParser;
use sauce::ast::ast::Ast;

fn main() {
    let src_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "example.sauce".to_string());

    let src = std::fs::read_to_string(&src_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", src_path));

    let lexer = Lexer::new(&src);
    let mut tokens: Vec<SpannedToken> = Vec::new();

    for item in lexer {
        match item {
            Ok(tok) => tokens.push(tok),
            Err(e) => {
                eprintln!("lex error: {e}");
                return;
            }
        }
    }

    let parser = parser_statement()
        .repeated()
        .collect::<Vec<_>>();

    let result = parser.parse(tokens.as_slice()).into_result();

    match result {
        Ok(items) => {
            let ast = Ast { items };
            println!("{:#?}", ast);
        }
        Err(errors) => {
            for err in errors {
                eprintln!("parse error: {err}");
            }
        }
    }
}
