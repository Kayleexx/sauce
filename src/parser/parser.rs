use crate::ast::ast::{Ast, Expr, Statement};
use crate::errors::parse::ParseError;
use crate::lexer::{SpannedToken, Token};

use chumsky::prelude::*;
use chumsky::select;

pub struct SauceParser;

impl SauceParser {
    pub fn new() -> Self {
        SauceParser
    }

    pub fn parse(&self, tokens: &[SpannedToken]) -> Result<Ast, ParseError> {
        let stmts_parser = parser_statement()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(end());

        let parse_result = stmts_parser.parse(tokens);
        let result = parse_result.into_result();

        match result {
            Ok(stmts) => Ok(Ast { items: stmts }),
            Err(errors) => {
                let first = errors
                    .into_iter()
                    .next()
                    .expect("parser reported no errors but returned Err");

                Err(Self::map_error(first))
            }
        }
    }

    fn map_error<E: std::fmt::Display>(err: E) -> ParseError {
        ParseError::Generic(err.to_string())
    }
}

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

pub fn parser_string<'src>() -> impl Parser<'src, &'src [SpannedToken], Expr> + Clone {
    select! { SpannedToken { token: Token::String(value), ..} => Expr::String(value) }
}
pub fn parser_expr<'src>() -> impl Parser<'src, &'src [SpannedToken], Expr> + Clone {
    recursive(|expr| {
        let toss_kw = select! {
            SpannedToken { token: Token::Toss, .. } => (),
        };

        let toss_expr = toss_kw
            .ignore_then(parser_name())
            .then(expr.clone().or_not())
            .map(|(effect, arg)| Expr::Toss {
                effect,
                arg: arg.map(Box::new),
            });

        let atom_base = parser_integer()
            .or(parser_ident())
            .or(parser_string())
            .or(toss_expr);

        let lparen = select! {
            SpannedToken { token: Token::LParen, .. } => (),
        };
        let rparen = select! {
            SpannedToken { token: Token::RParen, .. } => (),
        };

        let paren_expr = lparen.ignore_then(expr.clone()).then_ignore(rparen);

        let atom = atom_base.or(paren_expr);
        let atom_for_pipe = atom.clone();

        let pipe = select! {
            SpannedToken { token: Token::Pipe, .. } => (),
        };

        atom.foldl(pipe.ignore_then(atom_for_pipe).repeated(), |left, right| {
            Expr::Pipeline(Box::new(left), Box::new(right))
        })
    })
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

fn parser_expr_stmt<'src>() -> impl Parser<'src, &'src [SpannedToken], Statement> + Clone {
    let semi = select! {
        SpannedToken { token: Token::Semicolon, .. } => (),
    };

    parser_expr().then_ignore(semi).map(Statement::ExprStmt)
}

pub fn parser_statement<'src>() -> impl Parser<'src, &'src [SpannedToken], Statement> + Clone {
    parser_let().or(parser_yell()).or(parser_expr_stmt())
}
