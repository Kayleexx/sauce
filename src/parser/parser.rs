use crate::lexer::{SpannedToken, Token};
use crate::errors::parse::ParseError;
use crate::ast::ast::{Expr, Statement};
use chumsky::prelude::*;


pub fn parser_integer<'a>() -> impl Parser<'a, &'a [SpannedToken], Expr, Simple<SpannedToken>> {
    select! {
        SpannedToken { token: Token::Int(value), ..} => Expr::Int(value),
    }
}

pub fn parser_ident<'a>() -> impl Parser<'a, &'a[SpannedToken], Expr, Simple<SpannedToken>> {
    select!{
        SpannedToken { token: Token::Ident(name), ..} => Expr::Ident(name),
    }
}

pub fn parser_expr<'a>() -> impl Parser<'a, &'a[SpannedToken], Expr, Simple<SpannedToken>> {
    let atom = parser_integer().or(parser_ident());
        let pipe_op = select! {
            SpannedToken { token: Token::Pipe, ..} => (),
        };
        atom.clone().then(pipe_op.ignore_then(atom).repeated())
        .foldl(|left, right| Expr::Pipeline(Box::new(left), Box::new(right)))
}

pub fn parser_yell<'a>() -> impl Parser<'a, &'a [SpannedToken], Statement, Simple<SpannedToken>> {
    let yell_kw = select! {
        SpannedToken { token: Token::Yell, .. } => (),
    };

    let semi = select! {
        SpannedToken { token: Token::Semicolon, .. } => (),
    };

    yell_kw.ignore_then(parser_expr())
        .then_ignore(semi)
        .map(|expr| Statement::Yell { expr })
}


pub fn parser_name<'a>() -> impl Parser<'a, &'a[SpannedToken], String, Simple<SpannedToken>> {
    select!{
        SpannedToken { token: Token::Ident(name), ..} => name,
    }
}

fn parser_let<'a>() -> impl Parser<'a, &'a[SpannedToken], Statement, Simple<SpannedToken>> {
    let grab_kw = select! {
        SpannedToken { token: Token::Grab, ..} => (),
    };
    let equals = select! {
        SpannedToken { token: Token::Equals, ..} => (),
    };
    let semi = select! {
        SpannedToken { token: Token::Semicolon, ..} => (),
    };
    grab_kw
        .ignore_then(parser_name())
        .then_ignore(equals)
        .then(parser_expr())
        .then_ignore(semi)
        .map(|(name, expr)| Statement::Let { name, expr })

}

pub fn parser_statement<'a>() -> impl Parser<'a, &'a[SpannedToken], Statement, Simple<SpannedToken>> {
    parser_let().or(parser_yell())
}
