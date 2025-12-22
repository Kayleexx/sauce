use crate::ast::ast::Statement;
use crate::codegen::{context::Codegen, expr::codegen_expr};
use inkwell::values::PointerValue;
use std::collections::HashMap;

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
                .build_alloca(value.get_type(), name)
                .expect("alloca failed");

            cg.builder.build_store(ptr, value).expect("store failed");

            env.vars.insert(name.clone(), ptr);
        }

        Statement::Yell { expr } => {
            let value = codegen_expr(cg, env, expr);

            let fmt_str = match value {
                inkwell::values::BasicValueEnum::IntValue(_) => "%ld\n",
                inkwell::values::BasicValueEnum::PointerValue(_) => "%s\n",
                _ => "%ld\n",
            };

            let fmt = cg
                .builder
                .build_global_string_ptr(fmt_str, "fmt")
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
            eprintln!(
                "codegen error: `toss` is not supported in the LLVM backend yet.\n\
Hint: use the interpreter backend to run programs with effects."
            );
            std::process::exit(1);
        }
    }
}
