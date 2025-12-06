use crate::lexer::{SpannedToken, Token};
use crate::ast::ast::{Expr, Statement};
use chumsky::prelude::*;

pub fn parser_integer<'src>() -> impl Parser<'src, &'src [SpannedToken], Expr> + Clone {
    select! {
        SpannedToken { token: Token::Int(value), .. } => Expr::Int(value),
    }
}

pub fn parser_ident<'src>() -> impl Parser<'src, &'src [SpannedToken], Expr> + Clone {
    select! {
        SpannedToken { token: Token::Ident(name), .. } => Expr::Ident(name),
    }
}

fn parser_atom<'src>() -> impl Parser<'src, &'src [SpannedToken], Expr> + Clone {
    parser_integer().or(parser_ident())
}

pub fn parser_expr<'src>() -> impl Parser<'src, &'src [SpannedToken], Expr> + Clone {
    let pipe = select! {
        SpannedToken { token: Token::Pipe, .. } => (),
    };

    parser_atom().foldl(
        pipe.ignore_then(parser_atom()).repeated(),
        |left, right| Expr::Pipeline(Box::new(left), Box::new(right)),
    )
}

pub fn parser_name<'src>() -> impl Parser<'src, &'src [SpannedToken], String> + Clone {
    select! {
        SpannedToken { token: Token::Ident(name), .. } => name,
    }
}

pub fn parser_yell<'src>() -> impl Parser<'src, &'src [SpannedToken], Statement> + Clone {
    let yell_kw = select! {
        SpannedToken { token: Token::Yell, .. } => (),
    };

    let semi = select! {
        SpannedToken { token: Token::Semicolon, .. } => (),
    };

    yell_kw
        .ignore_then(parser_expr())
        .then_ignore(semi)
        .map(|expr| Statement::Yell { expr })
}

fn parser_let<'src>() -> impl Parser<'src, &'src [SpannedToken], Statement> + Clone {
    let grab_kw = select! {
        SpannedToken { token: Token::Grab, .. } => (),
    };
    let equals = select! {
        SpannedToken { token: Token::Equals, .. } => (),
    };
    let semi = select! {
        SpannedToken { token: Token::Semicolon, .. } => (),
    };

    grab_kw
        .ignore_then(parser_name())
        .then_ignore(equals)
        .then(parser_expr())
        .then_ignore(semi)
        .map(|(name, expr)| Statement::Let { name, expr })
}

pub fn parser_statement<'src>() -> impl Parser<'src, &'src [SpannedToken], Statement> + Clone {
    parser_let().or(parser_yell())
}
