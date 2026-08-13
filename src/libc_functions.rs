use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use inkwell::AddressSpace;

pub struct LibcFunctions<'ctx> {
    pub printf_fn: FunctionValue<'ctx>,
    pub snprintf_fn: FunctionValue<'ctx>,
    pub strcmp_fn: FunctionValue<'ctx>,
    pub strlen_fn: FunctionValue<'ctx>,
    pub strcpy_fn: FunctionValue<'ctx>,
    pub strcat_fn: FunctionValue<'ctx>,
    pub malloc_fn: FunctionValue<'ctx>,
    pub atoll_fn: FunctionValue<'ctx>,
    pub atof_fn: FunctionValue<'ctx>,
    pub realloc_fn: FunctionValue<'ctx>,
    pub memcpy_fn: FunctionValue<'ctx>,
}

impl<'ctx> LibcFunctions<'ctx> {
    pub fn new(context: &'ctx Context, module: &Module<'ctx>) -> Self {
        let i32_type = context.i32_type();
        let i64_type = context.i64_type();
        let f64_type = context.f64_type();
        let str_type = context.ptr_type(AddressSpace::default());
        let ptr_type = context.ptr_type(AddressSpace::default());

        let printf_fn = module.add_function("printf", i32_type.fn_type(&[str_type.into()], true), None);
        let snprintf_fn = module.add_function(
            "snprintf",
            i32_type.fn_type(&[str_type.into(), i64_type.into(), str_type.into()], true),
            None,
        );
        let strcmp_fn = module.add_function("strcmp", i32_type.fn_type(&[str_type.into(), str_type.into()], false), None);
        let strlen_fn = module.add_function("strlen", i64_type.fn_type(&[str_type.into()], false), None);
        let strcpy_fn = module.add_function("strcpy", str_type.fn_type(&[str_type.into(), str_type.into()], false), None);
        let strcat_fn = module.add_function("strcat", str_type.fn_type(&[str_type.into(), str_type.into()], false), None);
        let malloc_fn = module.add_function("malloc", str_type.fn_type(&[i64_type.into()], false), None);
        let atoll_fn = module.add_function("atoll", i64_type.fn_type(&[str_type.into()], false), None);
        let atof_fn = module.add_function("atof", f64_type.fn_type(&[str_type.into()], false), None);
        let realloc_fn = module.add_function("realloc", ptr_type.fn_type(&[ptr_type.into(), i64_type.into()], false), None);
        let memcpy_fn = module.add_function(
            "memcpy",
            ptr_type.fn_type(&[ptr_type.into(), ptr_type.into(), i64_type.into()], false),
            None,
        );

        LibcFunctions {
            printf_fn,
            snprintf_fn,
            strcmp_fn,
            strlen_fn,
            strcpy_fn,
            strcat_fn,
            malloc_fn,
            atoll_fn,
            atof_fn,
            realloc_fn,
            memcpy_fn,
        }
    }
}
