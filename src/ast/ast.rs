#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub items: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement{
    ExprStmt(Expr),
    Let { name: String, expr: Expr },
    Yell { expr: Expr },
    Toss { expr: Expr },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {

    Ident(String),
    Int(i64),
    Pipeline(Box<Expr>, Box<Expr>),
}