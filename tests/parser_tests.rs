use sauce::ast::ast::{Expr, Statement};
use sauce::errors::parse::ParseError;
use sauce::lexer::Lexer;
use sauce::parser::SauceParser;

#[test]
fn parse_basic_pipeline() {
    let src = "grab x = 1 |> 2;";
    let lexer = Lexer::new(src);

    let tokens: Vec<_> = lexer.collect::<Result<_, _>>().expect("lexing failed");
    let parser = SauceParser::new();

    let ast = parser.parse(&tokens).expect("parsing should succeed");

    assert_eq!(ast.items.len(), 1);

    match &ast.items[0] {
        Statement::Let { name, expr } => {
            assert_eq!(name, "x");

            match expr {
                Expr::Pipeline(left, right) => {
                    assert!(matches!(**left, Expr::Int(1)));
                    assert!(matches!(**right, Expr::Int(2)));
                }
                other => panic!("expected pipeline, got {other:?}"),
            }
        }
        other => panic!("expected let stmt, got {other:?}"),
    }
}

#[test]
fn parse_incomplete_input() {
    let src = "grab x = 1 |>";
    let lexer = Lexer::new(src);
    let tokens: Vec<_> = lexer.collect::<Result<_, _>>().unwrap();

    let parser = SauceParser::new();
    let result = parser.parse(&tokens);

    assert!(matches!(result, Err(ParseError::Generic(_))));
}

#[test]
fn parse_parenthesized_pipeline() {
    let src = "grab x = (1 |> 2);";
    let tokens = Lexer::new(src)
        .collect::<Result<Vec<_>, _>>()
        .expect("lex ok");

    let parser = SauceParser::new();
    let ast = parser.parse(&tokens).expect("parse ok");

    assert_eq!(ast.items.len(), 1);
    match &ast.items[0] {
        Statement::Let { name, expr } => {
            assert_eq!(name, "x");
            match expr {
                Expr::Pipeline(left, right) => {
                    assert!(matches!(**left, Expr::Int(1)));
                    assert!(matches!(**right, Expr::Int(2)));
                }
                other => panic!("expected pipeline, got {other:?}"),
            }
        }
        other => panic!("expected let, got {other:?}"),
    }
}

#[test]
fn parse_string_literal() {
    let src = "yell \"sauce\";";
    let tokens = Lexer::new(src)
        .collect::<Result<Vec<_>, _>>()
        .expect("lex ok");

    let parser = SauceParser::new();
    let ast = parser.parse(&tokens).expect("parse ok");

    assert_eq!(ast.items.len(), 1);
    match &ast.items[0] {
        Statement::Yell { expr } => match expr {
            Expr::String(s) => assert_eq!(s, "\"sauce\""),
            other => panic!("expected string expr, got {other:?}"),
        },
        other => panic!("expected yell, got {other:?}"),
    }
}
