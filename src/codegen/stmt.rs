use std::collections::HashMap;

use inkwell::values::PointerValue;

use crate::ast::ast::Statement;
use crate::codegen::{context::Codegen, expr::codegen_expr};

pub struct LocalEnv<'ctx> {
    pub vars: HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> LocalEnv<'ctx> {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
}

pub fn codegen_stmt<'ctx>(cg: &mut Codegen<'ctx>, env: &mut LocalEnv<'ctx>, stmt: &Statement) {
    match stmt {
        Statement::Let { name, expr } => {
            let value = codegen_expr(cg, env, expr);
            let ptr = cg
                .builder
                .build_alloca(cg.context.i64_type(), name)
                .expect("alloca failed");

            cg.builder.build_store(ptr, value).expect("store failed");

            env.vars.insert(name.clone(), ptr);
        }

        Statement::Yell { expr } => {
            let value = codegen_expr(cg, env, expr);
            let fmt = cg
                .builder
                .build_global_string_ptr("%ld\n", "fmt")
                .expect("format string failed");

            cg.builder
                .build_call(
                    cg.printf,
                    &[fmt.as_pointer_value().into(), value.into()],
                    "printf_call",
                )
                .expect("printf call failed");
        }

        Statement::ExprStmt(expr) => {
            let _ = codegen_expr(cg, env, expr);
        }

        Statement::Toss { .. } => {
            panic!("toss is not supported in LLVM backend yet");
        }
    }
}
