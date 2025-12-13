use crate::errors::lex::LexError;
use crate::lexer::token::Token;
use crate::util::span::Span;
use logos::Logos;

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

impl SpannedToken {
    pub fn new(token: Token, span: Span) -> Self {
        Self { token, span }
    }
}

pub struct Lexer<'input> {
    inner: logos::Lexer<'input, Token>,
}

impl<'input> Lexer<'input> {
    pub fn new(src: &'input str) -> Self {
        Self {
            inner: Token::lexer(src),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<SpannedToken, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        let token_result = self.inner.next()?;
        let range = self.inner.span();
        let span = Span::new(range.start, range.end);

        match token_result {
            Ok(token) => Some(Ok(SpannedToken::new(token, span))),
            Err(_) => Some(Err(LexError::InvalidToken(span))),
        }
    }
}
