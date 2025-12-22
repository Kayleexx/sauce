use sauce::lexer::Lexer;
use sauce::parser::SauceParser;
use sauce::typechecker::typecheck_program;

#[test]
fn typecheck_simple_let() {
    let src = "grab x = 10; yell x;";
    let tokens = Lexer::new(src)
        .collect::<Result<Vec<_>, _>>()
        .expect("lex failed");

    let parser = SauceParser::new();
    let ast = parser.parse(&tokens).expect("parse failed");

    let result = typecheck_program(&ast);
    assert!(result.is_ok());
}
#[test]
fn typecheck_unknown_ident() {
    let src = "yell x;";
    let tokens = Lexer::new(src)
        .collect::<Result<Vec<_>, _>>()
        .expect("lex failed");

    let parser = SauceParser::new();
    let ast = parser.parse(&tokens).expect("parse failed");

    let result = typecheck_program(&ast);
    assert!(result.is_err());
}

#[test]
fn typecheck_toss() {
    let src = "toss oops \"bad\";";
    let tokens = Lexer::new(src)
        .collect::<Result<Vec<_>, _>>()
        .expect("lex failed");

    let parser = SauceParser::new();
    let ast = parser.parse(&tokens).expect("parse failed");

    let result = typecheck_program(&ast);
    assert!(result.is_ok());
}

#[test]
fn typecheck_pipeline_into_literal_should_fail() {
    let src = "grab x = 1 |> 2 |> 3;";
    let tokens = Lexer::new(src).collect::<Result<Vec<_>, _>>().unwrap();

    let ast = SauceParser::new().parse(&tokens).unwrap();
    let result = typecheck_program(&ast);

    assert!(result.is_err());
}
