use sauce::lexer::{Lexer, SpannedToken, Token};
use sauce::util::span::Span;
use sauce::errors::lex::LexError;

fn unwrap_ok_token(item: &Result<SpannedToken, LexError>) -> &SpannedToken {
    item.as_ref().expect("expected Ok token, got Err")
}

#[test]
fn basic_pipeline_lexing() {
    let src = "grab x = 10 |> yell";
    let lexer = Lexer::new(src);
    let tokens: Vec<_> = lexer.collect();
    assert_eq!(tokens.len(), 6);


    let t0 = unwrap_ok_token(&tokens[0]);
    assert_eq!(t0.token, Token::Grab);
    assert_eq!(t0.span, Span::new(0, 4)); // "grab"

    // t1: x (identifier)
    let t1 = unwrap_ok_token(&tokens[1]);
    assert_eq!(t1.span, Span::new(5, 6)); // "x"
    assert_eq!(t1.token, Token::Ident("x".to_string()));

    // t2: =
    let t2 = unwrap_ok_token(&tokens[2]);
    assert_eq!(t2.token, Token::Equals);
    assert_eq!(t2.span, Span::new(7, 8)); // "="

    // t3: 10
    let t3 = unwrap_ok_token(&tokens[3]);
    assert_eq!(t3.span, Span::new(9, 11)); // "10"
    assert_eq!(t3.token, Token::Int(10));

    // t4: |>
    let t4 = unwrap_ok_token(&tokens[4]);
    assert_eq!(t4.token, Token::Pipe);
    assert_eq!(t4.span, Span::new(12, 14)); // "|>"

    // t5: yell keyword
    let t5 = unwrap_ok_token(&tokens[5]);
    assert_eq!(t5.token, Token::Yell);
    assert_eq!(t5.span, Span::new(15, 19)); // "yell"
}

#[test]
fn unknown_character_produces_lex_error() {
    let src = "grab @";
    let lexer = Lexer::new(src);
    let tokens: Vec<_> = lexer.collect();

    assert_eq!(tokens.len(), 2);
    assert!(tokens[0].is_ok());

    match &tokens[1] {
        Ok(_) => panic!("expected error on second token, got Ok"),
        Err(e) => match e {
            LexError::InvalidToken(span) => {
                assert_eq!(*span, Span::new(5, 6)); // "@" at byte 5
            }
            other => panic!("expected InvalidToken, got {other:?}"),
        },
    }
}

#[test]
fn lexer_skips_whitespace() {
    let src = "  grab   \n  x";
    let lexer = Lexer::new(src);
    let tokens: Vec<_> = lexer.collect();

    // should only see: grab, x
    assert_eq!(tokens.len(), 2);

    let t0 = unwrap_ok_token(&tokens[0]);
    assert_eq!(t0.token, Token::Grab);

    let t1 = unwrap_ok_token(&tokens[1]);
    match &t1.token {
        Token::Ident(name) => assert_eq!(name, "x"),
        other => panic!("expected ident 'x', got {other:?}"),
    }
}
