mod modules;
mod system;

use std::io::{Error, ErrorKind};
use std::rc::Rc;

use crate::modules::lib_module::LibModule;
use crate::modules::load_modules;
use crate::system::cli_args::CliArgs;

use shida_core::ffi;
use shida_core::module::CanHandleFunc;


fn assign_lib(lib: &mut Option<Rc<LibModule>>, lib_candidate: &Rc<LibModule>, lib_name_arg: Option<&String>) {
    let can_handle_fn: CanHandleFunc = lib_candidate.module.can_handle;
    let input_type = match lib_name_arg {
        Some(i) => i,
        None => return,
    };
    unsafe {
        if can_handle_fn(ffi::string_to_ccharptr(input_type.to_string())) {
            *lib = Some(lib_candidate.clone());
        }
    }
}

fn main() -> Result<(), Error> {
    let args = match CliArgs::new_from_args() {
        Ok(a) => a,
        Err(e) => return Err(Error::new(ErrorKind::InvalidInput, format!("Failed to parse arguments: {}", e))),
    };

    let mut modules = load_modules().into_iter();
    let mut input_module: Option<Rc<LibModule>> = None;
    let mut output_module: Option<Rc<LibModule>> = None;
    while let Some(lm) = modules.next() {
        assign_lib(&mut input_module, &lm, args.input.get("type"));
        assign_lib(&mut output_module, &lm, args.output.get("type"));
    }

    let input_module = match input_module {
        None => return Err(Error::new(ErrorKind::InvalidInput, "Cannot find a module for input.")),
        Some(m) => m,
    };
    // let output_module = match output_module {
    //     None => return Err(Error::new(ErrorKind::InvalidInput, "Cannot find a module for output.")),
    //     Some(m) => m,
    // };

    let init_reader_fn = input_module.module.init_reader;
    let src_read_fn = input_module.module.read;

    let (len, input_args_cchar_ptr) = args.input_as_c_char_ptr();
    let (handle, err) = init_reader_fn(len, input_args_cchar_ptr as *const ffi::ConstCCharPtr);
    if std::ptr::null() == err {
        println!("No error");
    } else {
        let e = unsafe {
            match ffi::ccharptr_to_string(err) {
                Ok(r) => r,
                Err(_) => String::from("Failed to decode an error message."),
            }
        };
        return Err(Error::new(ErrorKind::InvalidInput, e));
    }

    let (result, err) = src_read_fn(handle);
    if std::ptr::null() == err {
        let res = unsafe { match ffi::ccharptr_to_string(result) {
            Ok(r) => r,
            Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Failed.")),
        }};
        println!("{}", res);
    } else {
        let e = unsafe {
            match ffi::ccharptr_to_string(err) {
                Ok(r) => r,
                Err(_) => String::from("Failed to decode an error message."),
            }
        };
        return Err(Error::new(ErrorKind::InvalidInput, e))
    }

    Ok(())
}
