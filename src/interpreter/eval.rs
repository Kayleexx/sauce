use super::{effects::Effect, env::RuntimeEnv, error::RuntimeError, value::Value};
use crate::ast::ast::{Ast, Expr, Statement};

type EvalResult<T> = Result<T, Control>;

#[derive(Debug)]
enum Control {
    Effect(Effect),
    Error(RuntimeError),
}

impl From<RuntimeError> for Control {
    fn from(e: RuntimeError) -> Self {
        Control::Error(e)
    }
}

fn eval_expr(expr: &Expr, env: &mut RuntimeEnv) -> EvalResult<Value> {
    match expr {
        Expr::Int(n) => Ok(Value::Int(*n)),
        Expr::String(s) => Ok(Value::String(s.clone())),

        Expr::Ident(name) => env
            .get(name)
            .ok_or_else(|| RuntimeError::UnknownVariable(name.clone()).into()),

        Expr::Pipeline(lhs, rhs) => {
            let value = eval_expr(lhs, env)?;

            // pipeline creates a new scope with `_`
            let mut next_env = env.clone();
            next_env.set("_", value);

            eval_expr(rhs, &mut next_env)
        }

        Expr::Toss { effect, arg } => {
            let payload = if let Some(expr) = arg {
                Some(eval_expr(expr, env)?)
            } else {
                None
            };

            Err(Control::Effect(Effect {
                name: effect.clone(),
                payload,
            }))
        }
    }
}

fn eval_stmt(stmt: &Statement, env: &mut RuntimeEnv) -> EvalResult<()> {
    match stmt {
        Statement::Let { name, expr } => {
            let val = eval_expr(expr, env)?;
            env.set(name.clone(), val);
            Ok(())
        }

        Statement::Yell { expr } => {
            let val = eval_expr(expr, env)?;
            println!("{:?}", val);
            Ok(())
        }

        Statement::ExprStmt(expr) => {
            eval_expr(expr, env)?;
            Ok(())
        }

        Statement::Toss { expr } => {
            eval_expr(expr, env)?;
            Ok(())
        }
    }
}

pub fn eval_program(ast: &Ast) -> Result<(), RuntimeError> {
    let mut env = RuntimeEnv::new();

    for stmt in &ast.items {
        match eval_stmt(stmt, &mut env) {
            Ok(_) => {}
            Err(Control::Error(e)) => return Err(e),
            Err(Control::Effect(e)) => return Err(RuntimeError::UnhandledEffect(e.name)),
        }
    }

    Ok(())
}
