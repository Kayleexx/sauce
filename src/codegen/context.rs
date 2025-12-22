use inkwell::{
    AddressSpace, builder::Builder, context::Context, module::Module, values::FunctionValue,
};

pub struct Codegen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub current_fn: FunctionValue<'ctx>,
    pub printf: FunctionValue<'ctx>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(context: &'ctx Context, name: &str) -> Self {
        let module = context.create_module(name);
        let builder = context.create_builder();

        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = module.add_function("main", fn_type, None);

        let entry = context.append_basic_block(function, "entry");
        builder.position_at_end(entry);

        let i8ptr = context.ptr_type(AddressSpace::default());

        let printf_type = context.i32_type().fn_type(&[i8ptr.into()], true);

        let printf = module.add_function("printf", printf_type, None);

        Self {
            context,
            module,
            builder,
            current_fn: function,
            printf,
        }
    }
}
