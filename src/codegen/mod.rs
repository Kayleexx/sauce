pub mod context;
pub mod expr;
pub mod llvm;
pub mod runtime;
pub mod stmt;
pub mod types;

use crate::ast::ast::Ast;
use inkwell::context::Context;
use stmt::{LocalEnv, codegen_stmt};

pub fn codegen(ast: &Ast) {
    let context = Context::create();
    let mut cg = context::Codegen::new(&context, "sauce");
    let mut env = LocalEnv::new();

    for stmt in &ast.items {
        codegen_stmt(&mut cg, &mut env, stmt);
    }

    let _ = cg
        .builder
        .build_return(Some(&context.i32_type().const_int(0, false)));

    cg.module
        .print_to_file("out.ll")
        .expect("failed to write LLVM IR");
}
