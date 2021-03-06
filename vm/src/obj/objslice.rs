use num_bigint::BigInt;

use crate::function::PyFuncArgs;
use crate::pyobject::{PyContext, PyObjectRef, PyRef, PyResult, PyValue, TypeProtocol};
use crate::vm::VirtualMachine;

use super::objint;
use crate::obj::objtype::PyClassRef;

#[derive(Debug)]
pub struct PySlice {
    // TODO: should be private
    pub start: Option<BigInt>,
    pub stop: Option<BigInt>,
    pub step: Option<BigInt>,
}

impl PyValue for PySlice {
    fn class(vm: &VirtualMachine) -> PyClassRef {
        vm.ctx.slice_type()
    }
}

pub type PySliceRef = PyRef<PySlice>;

fn slice_new(vm: &VirtualMachine, args: PyFuncArgs) -> PyResult {
    no_kwargs!(vm, args);
    let (cls, start, stop, step): (
        &PyObjectRef,
        Option<&PyObjectRef>,
        Option<&PyObjectRef>,
        Option<&PyObjectRef>,
    ) = match args.args.len() {
        0 | 1 => Err(vm.new_type_error("slice() must have at least one arguments.".to_owned())),
        2 => {
            arg_check!(
                vm,
                args,
                required = [
                    (cls, Some(vm.ctx.type_type())),
                    (stop, Some(vm.ctx.int_type()))
                ]
            );
            Ok((cls, None, Some(stop), None))
        }
        _ => {
            arg_check!(
                vm,
                args,
                required = [
                    (cls, Some(vm.ctx.type_type())),
                    (start, Some(vm.ctx.int_type())),
                    (stop, Some(vm.ctx.int_type()))
                ],
                optional = [(step, Some(vm.ctx.int_type()))]
            );
            Ok((cls, Some(start), Some(stop), step))
        }
    }?;
    PySlice {
        start: start.map(|x| objint::get_value(x).clone()),
        stop: stop.map(|x| objint::get_value(x).clone()),
        step: step.map(|x| objint::get_value(x).clone()),
    }
    .into_ref_with_type(vm, cls.clone().downcast().unwrap())
    .map(|x| x.into_object())
}

fn get_property_value(vm: &VirtualMachine, value: &Option<BigInt>) -> PyResult {
    if let Some(value) = value {
        Ok(vm.ctx.new_int(value.clone()))
    } else {
        Ok(vm.get_none())
    }
}

fn slice_start(vm: &VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(slice, Some(vm.ctx.slice_type()))]);
    if let Some(PySlice { start, .. }) = &slice.payload() {
        get_property_value(vm, start)
    } else {
        panic!("Slice has incorrect payload.");
    }
}

fn slice_stop(vm: &VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(slice, Some(vm.ctx.slice_type()))]);
    if let Some(PySlice { stop, .. }) = &slice.payload() {
        get_property_value(vm, stop)
    } else {
        panic!("Slice has incorrect payload.");
    }
}

fn slice_step(vm: &VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(slice, Some(vm.ctx.slice_type()))]);
    if let Some(PySlice { step, .. }) = &slice.payload() {
        get_property_value(vm, step)
    } else {
        panic!("Slice has incorrect payload.");
    }
}

pub fn init(context: &PyContext) {
    let slice_type = &context.slice_type;

    extend_class!(context, slice_type, {
        "__new__" => context.new_rustfunc(slice_new),
        "start" => context.new_property(slice_start),
        "stop" => context.new_property(slice_stop),
        "step" => context.new_property(slice_step)
    });
}
