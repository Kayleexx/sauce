use crate::ast::ast::Expr;
use crate::codegen::{context::Codegen, stmt::LocalEnv};
use inkwell::values::BasicValueEnum;

pub fn codegen_expr<'ctx>(
    cg: &mut Codegen<'ctx>,
    env: &mut LocalEnv<'ctx>,
    expr: &Expr,
) -> BasicValueEnum<'ctx> {
    match expr {
        Expr::Int(n) => cg.context.i64_type().const_int(*n as u64, true).into(),

        Expr::Ident(name) => {
            let ptr = env.vars.get(name).expect("unknown variable");
            cg.builder
                .build_load(cg.context.i64_type(), *ptr, "loadtmp")
                .expect("load failed")
        }

        Expr::Pipeline(lhs, rhs) => {
            // 1. eval lhs
            let lhs_val = codegen_expr(cg, env, lhs);

            // 2. bind `_`
            let tmp = cg
                .builder
                .build_alloca(cg.context.i64_type(), "_")
                .expect("alloca _ failed");

            cg.builder
                .build_store(tmp, lhs_val)
                .expect("store _ failed");

            // shadow `_` only
            let old = env.vars.insert("_".to_string(), tmp);

            // 3. eval rhs
            let result = codegen_expr(cg, env, rhs);

            // restore previous `_` if any
            if let Some(prev) = old {
                env.vars.insert("_".to_string(), prev);
            } else {
                env.vars.remove("_");
            }

            result
        }

        Expr::Toss { .. } => {
            panic!("toss is not supported in LLVM backend yet");
        }

        Expr::String(_) => {
            panic!("string literals not supported in LLVM backend yet");
        }
    }
}
