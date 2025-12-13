use std::collections::HashMap;
use crate::ast::ast::{Expr, Ast};
use crate::typecheck::{types::Type, errors::TypeError};


#[derive(Debug)]
pub struct TypeEnv {
    vars: HashMap<String, Type>,
}

impl TypeEnv {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
    pub fn insert(&mut self, name: String, ty: Type) {
        self.vars.insert(name, ty);
    }
    pub fn get(&self, name: &str) -> Option<&Type> {
        self.vars.get(name)
    }
}

pub fn typecheck_expr(env: &TypeEnv, expr: &Expr) -> Result<Type, TypeError> {
    match expr {
        Expr::Int(_) => Ok(Type::Int),

        Expr::String(_) => Ok(Type::String),

        Expr::Ident(name) => {
            match env.get(name) {
                Some(ty) => Ok(ty.clone()),
                None => Err(TypeError::Generic(format!(
                    "unknown identifier `{}`",
                    name
                ))),
            }
        }

        Expr::Toss { .. } => Ok(Type::Unit),

        Expr::Pipeline(_, right) => {
            typecheck_expr(env, right)
        }
    }
}


pub fn typecheck_program(ast: &Ast) -> Result<(), TypeError> {
    let mut env = TypeEnv::new();

    for stmt in &ast.items {
        typecheck_stmt(&mut env, stmt)?;
    }

    Ok(())
}
