use crate::function::PyFuncArgs;
use crate::obj::objsequence;
use crate::obj::objtype;
use crate::obj::objtype::PyClassRef;
use crate::pyobject::{create_type, PyContext, PyObjectRef, PyResult, TypeProtocol};
use crate::vm::VirtualMachine;

fn exception_init(vm: &VirtualMachine, args: PyFuncArgs) -> PyResult {
    let zelf = args.args[0].clone();
    let msg = if args.args.len() > 1 {
        args.args[1].clone()
    } else {
        vm.new_str("No msg".to_string())
    };
    let traceback = vm.ctx.new_list(Vec::new());
    vm.ctx.set_attr(&zelf, "msg", msg);
    vm.ctx.set_attr(&zelf, "__traceback__", traceback);
    Ok(vm.get_none())
}

// Print exception including traceback:
pub fn print_exception(vm: &VirtualMachine, exc: &PyObjectRef) {
    if let Ok(tb) = vm.get_attribute(exc.clone(), "__traceback__") {
        println!("Traceback (most recent call last):");
        if objtype::isinstance(&tb, &vm.ctx.list_type()) {
            let mut elements = objsequence::get_elements(&tb).to_vec();
            elements.reverse();
            for element in elements.iter() {
                if objtype::isinstance(&element, &vm.ctx.tuple_type()) {
                    let element = objsequence::get_elements(&element);
                    let filename = if let Ok(x) = vm.to_str(&element[0]) {
                        x.value.clone()
                    } else {
                        "<error>".to_string()
                    };

                    let lineno = if let Ok(x) = vm.to_str(&element[1]) {
                        x.value.clone()
                    } else {
                        "<error>".to_string()
                    };

                    let obj_name = if let Ok(x) = vm.to_str(&element[2]) {
                        x.value.clone()
                    } else {
                        "<error>".to_string()
                    };

                    println!("  File {}, line {}, in {}", filename, lineno, obj_name);
                } else {
                    println!("  File ??");
                }
            }
        }
    } else {
        println!("No traceback set on exception");
    }

    match vm.to_str(exc) {
        Ok(txt) => println!("{}", txt.value),
        Err(err) => println!("Error during error {:?}", err),
    }
}

fn exception_str(vm: &VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [(exc, Some(vm.ctx.exceptions.exception_type.clone()))]
    );
    let msg = if let Ok(m) = vm.get_attribute(exc.clone(), "msg") {
        match vm.to_pystr(&m) {
            Ok(msg) => msg,
            _ => "<exception str() failed>".to_string(),
        }
    } else {
        panic!("Error message must be set");
    };
    let s = format!("{}: {}", exc.class().name, msg);
    Ok(vm.new_str(s))
}

#[derive(Debug)]
pub struct ExceptionZoo {
    pub arithmetic_error: PyClassRef,
    pub assertion_error: PyClassRef,
    pub attribute_error: PyClassRef,
    pub base_exception_type: PyClassRef,
    pub exception_type: PyClassRef,
    pub file_not_found_error: PyClassRef,
    pub import_error: PyClassRef,
    pub index_error: PyClassRef,
    pub key_error: PyClassRef,
    pub module_not_found_error: PyClassRef,
    pub name_error: PyClassRef,
    pub not_implemented_error: PyClassRef,
    pub os_error: PyClassRef,
    pub overflow_error: PyClassRef,
    pub permission_error: PyClassRef,
    pub runtime_error: PyClassRef,
    pub stop_iteration: PyClassRef,
    pub syntax_error: PyClassRef,
    pub type_error: PyClassRef,
    pub value_error: PyClassRef,
    pub zero_division_error: PyClassRef,
}

impl ExceptionZoo {
    pub fn new(type_type: &PyClassRef, object_type: &PyClassRef) -> Self {
        // Sorted By Hierarchy then alphabetized.
        let base_exception_type = create_type("BaseException", &type_type, &object_type);
        let exception_type = create_type("Exception", &type_type, &base_exception_type);
        let arithmetic_error = create_type("ArithmeticError", &type_type, &exception_type);
        let assertion_error = create_type("AssertionError", &type_type, &exception_type);
        let attribute_error = create_type("AttributeError", &type_type, &exception_type);
        let import_error = create_type("ImportError", &type_type, &exception_type);
        let index_error = create_type("IndexError", &type_type, &exception_type);
        let key_error = create_type("KeyError", &type_type, &exception_type);
        let name_error = create_type("NameError", &type_type, &exception_type);
        let os_error = create_type("OSError", &type_type, &exception_type);
        let runtime_error = create_type("RuntimeError", &type_type, &exception_type);
        let stop_iteration = create_type("StopIteration", &type_type, &exception_type);
        let syntax_error = create_type("SyntaxError", &type_type, &exception_type);
        let type_error = create_type("TypeError", &type_type, &exception_type);
        let value_error = create_type("ValueError", &type_type, &exception_type);
        let overflow_error = create_type("OverflowError", &type_type, &arithmetic_error);
        let zero_division_error = create_type("ZeroDivisionError", &type_type, &arithmetic_error);
        let module_not_found_error = create_type("ModuleNotFoundError", &type_type, &import_error);
        let not_implemented_error = create_type("NotImplementedError", &type_type, &runtime_error);
        let file_not_found_error = create_type("FileNotFoundError", &type_type, &os_error);
        let permission_error = create_type("PermissionError", &type_type, &os_error);

        ExceptionZoo {
            arithmetic_error,
            assertion_error,
            attribute_error,
            base_exception_type,
            exception_type,
            file_not_found_error,
            import_error,
            index_error,
            key_error,
            module_not_found_error,
            name_error,
            not_implemented_error,
            os_error,
            overflow_error,
            permission_error,
            runtime_error,
            stop_iteration,
            syntax_error,
            type_error,
            value_error,
            zero_division_error,
        }
    }
}

pub fn init(context: &PyContext) {
    let base_exception_type = &context.exceptions.base_exception_type;
    extend_class!(context, base_exception_type, {
        "__init__" => context.new_rustfunc(exception_init)
    });

    let exception_type = &context.exceptions.exception_type;
    extend_class!(context, exception_type, {
        "__str__" => context.new_rustfunc(exception_str)
    });
}
