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
    pub usleep_fn: FunctionValue<'ctx>,

    // files
    pub fopen_fn: FunctionValue<'ctx>,
    pub fclose_fn: FunctionValue<'ctx>,
    pub fread_fn: FunctionValue<'ctx>,
    pub fwrite_fn: FunctionValue<'ctx>,
    pub fseek_fn: FunctionValue<'ctx>,
    pub ftell_fn: FunctionValue<'ctx>,
    pub remove_fn: FunctionValue<'ctx>,
    pub access_fn: FunctionValue<'ctx>,

    // stdin
    pub read_fn: FunctionValue<'ctx>,

    // sockets (Linux x86-64 / glibc)
    pub socket_fn: FunctionValue<'ctx>,
    pub bind_fn: FunctionValue<'ctx>,
    pub listen_fn: FunctionValue<'ctx>,
    pub accept_fn: FunctionValue<'ctx>,
    pub recv_fn: FunctionValue<'ctx>,
    pub send_fn: FunctionValue<'ctx>,
    pub close_fn: FunctionValue<'ctx>,
    pub connect_fn: FunctionValue<'ctx>,
    pub inet_addr_fn: FunctionValue<'ctx>,
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
        let usleep_fn = module.add_function("usleep", i32_type.fn_type(&[i32_type.into()], false), None);
        let fopen_fn = module.add_function("fopen", str_type.fn_type(&[str_type.into(), str_type.into()], false), None);
        let fclose_fn = module.add_function("fclose", i32_type.fn_type(&[str_type.into()], false), None);
        let fread_fn = module.add_function(
            "fread",
            i64_type.fn_type(&[str_type.into(), i64_type.into(), i64_type.into(), str_type.into()], false),
            None,
        );
        let fwrite_fn = module.add_function(
            "fwrite",
            i64_type.fn_type(&[str_type.into(), i64_type.into(), i64_type.into(), str_type.into()], false),
            None,
        );
        let fseek_fn = module.add_function(
            "fseek",
            i32_type.fn_type(&[str_type.into(), i64_type.into(), i32_type.into()], false),
            None,
        );
        let ftell_fn = module.add_function("ftell", i64_type.fn_type(&[str_type.into()], false), None);
        let remove_fn = module.add_function("remove", i32_type.fn_type(&[str_type.into()], false), None);
        let access_fn = module.add_function("access", i32_type.fn_type(&[str_type.into(), i32_type.into()], false), None);

        let read_fn = module.add_function(
            "read",
            i64_type.fn_type(&[i32_type.into(), str_type.into(), i64_type.into()], false),
            None,
        );

        let socket_fn = module.add_function(
            "socket",
            i32_type.fn_type(&[i32_type.into(), i32_type.into(), i32_type.into()], false),
            None,
        );
        let bind_fn = module.add_function(
            "bind",
            i32_type.fn_type(&[i32_type.into(), str_type.into(), i32_type.into()], false),
            None,
        );
        let listen_fn = module.add_function("listen", i32_type.fn_type(&[i32_type.into(), i32_type.into()], false), None);
        let accept_fn = module.add_function(
            "accept",
            i32_type.fn_type(&[i32_type.into(), str_type.into(), str_type.into()], false),
            None,
        );
        let recv_fn = module.add_function(
            "recv",
            i64_type.fn_type(&[i32_type.into(), str_type.into(), i64_type.into(), i32_type.into()], false),
            None,
        );
        let send_fn = module.add_function(
            "send",
            i64_type.fn_type(&[i32_type.into(), str_type.into(), i64_type.into(), i32_type.into()], false),
            None,
        );
        let close_fn = module.add_function("close", i32_type.fn_type(&[i32_type.into()], false), None);
        let connect_fn = module.add_function(
            "connect",
            i32_type.fn_type(&[i32_type.into(), str_type.into(), i32_type.into()], false),
            None,
        );
        let inet_addr_fn = module.add_function(
            "inet_addr",
            i32_type.fn_type(&[str_type.into()], false), // in_addr_t (u32) - w LLVM IR to i32 bitowo
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
            usleep_fn,
            accept_fn,
            access_fn,
            bind_fn,
            close_fn,
            fclose_fn,
            fopen_fn,
            fread_fn,
            fseek_fn,
            ftell_fn,
            fwrite_fn,
            listen_fn,
            read_fn,
            recv_fn,
            remove_fn,
            send_fn,
            socket_fn,
            connect_fn,
            inet_addr_fn,
        }
    }
}
