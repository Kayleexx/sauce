use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token {
    #[token("grab")]
    Grab,
    #[token("yell")]
    Yell,
    #[token("toss")]
    Toss,
    #[token("|>")]
    Pipe,

    #[token("=")]
    Equals,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    Int(i64),

    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#, |lex| {
    let s = lex.slice();
    s[1..s.len() - 1].to_string()})]
    String(String),

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
}
