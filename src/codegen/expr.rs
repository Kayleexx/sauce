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
            let lhs_val = codegen_expr(cg, env, lhs);

            let tmp = cg
                .builder
                .build_alloca(cg.context.i64_type(), "_")
                .expect("alloca _ failed");

            cg.builder
                .build_store(tmp, lhs_val)
                .expect("store _ failed");

            let old = env.vars.insert("_".to_string(), tmp);
            let result = codegen_expr(cg, env, rhs);

            if let Some(prev) = old {
                env.vars.insert("_".to_string(), prev);
            } else {
                env.vars.remove("_");
            }

            result
        }
        Expr::Toss { .. } => {
            eprintln!(
                "codegen error: `toss` is not supported in the LLVM backend yet.\n\
         Hint: use the interpreter backend to run programs with effects."
            );
            std::process::exit(1);
        }

        Expr::String(s) => {
            let global = cg.builder.build_global_string_ptr(s, "str");

            global
                .expect("failed to create string literal")
                .as_pointer_value()
                .into()
        }
    }
}
