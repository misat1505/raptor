use inkwell::context::Context;

use crate::backend::llvm::libc_functions::LibcFunctions;

mod arithmetic;
mod cast;
mod comparison;
mod llvm_value;
mod logical;
mod unary;

fn setup<'ctx>(context: &'ctx Context) -> (inkwell::module::Module<'ctx>, inkwell::builder::Builder<'ctx>, LibcFunctions<'ctx>) {
    let module = context.create_module("test_alu");
    let builder = context.create_builder();

    // Create a dummy function so the builder has an insertion point.
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let entry = context.append_basic_block(function, "entry");
    builder.position_at_end(entry);

    let libc = LibcFunctions::new(context, &module);

    (module, builder, libc)
}
