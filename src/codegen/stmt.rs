use inkwell::values::PointerValue;
use std::collections::HashMap;

use crate::ast::ast::{Expr, Statement};
use crate::codegen::context::Codegen;

pub struct LocalEnv<'ctx> {
    vars: HashMap<String, PointerValue<'ctx>>,
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
            if let Expr::Int(n) = expr {
                let i64_ty = cg.context.i64_type();

                let ptr = cg
                    .builder
                    .build_alloca(i64_ty, name)
                    .expect("alloca failed");

                cg.builder
                    .build_store(ptr, i64_ty.const_int(*n as u64, true))
                    .expect("store failed");

                env.vars.insert(name.clone(), ptr);
            }
        }

        Statement::Yell { expr } => {
            if let Expr::Ident(name) = expr {
                let ptr = env.vars.get(name).expect("unknown variable");

                let i64_ty = cg.context.i64_type();
                let val = cg
                    .builder
                    .build_load(i64_ty, *ptr, "loadtmp")
                    .expect("load failed");

                let fmt = cg
                    .builder
                    .build_global_string_ptr("%ld\n", "fmt")
                    .expect("string ptr failed");

                cg.builder
                    .build_call(
                        cg.printf,
                        &[fmt.as_pointer_value().into(), val.into()],
                        "printf_call",
                    )
                    .expect("printf call failed");
            }
        }

        _ => {}
    }
}
