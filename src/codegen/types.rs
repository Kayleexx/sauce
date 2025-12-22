use crate::typechecker::types::Type;
use inkwell::types::BasicTypeEnum;

pub fn llvm_type<'ctx>(
    ctx: &'ctx inkwell::context::Context,
    ty: &Type,
) -> Option<BasicTypeEnum<'ctx>> {
    match ty {
        Type::Int => Some(ctx.i64_type().into()),
        Type::String => Some(ctx.ptr_type(Default::default()).into()),
        Type::Unit => None,
    }
}
