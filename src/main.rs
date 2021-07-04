mod ffi;
mod modules;
mod system;

use std::rc::Rc;
use log::{error};

use shida_core::ffi::casting;
use shida_core::ffi::typedefs;

use crate::ffi::app_config;
use crate::modules::lib_module::LibModule;
use crate::modules::load_modules;
use crate::system::cli_args::CliArgs;
use crate::system::init;


fn assign_lib(lib: &mut Option<Rc<LibModule>>, lib_candidate: &Rc<LibModule>, lib_name_arg: Option<&String>) {
    let input_type = match lib_name_arg {
        Some(i) => i,
        None => return,
    };
    if (lib_candidate.module.can_handle)(casting::string_to_ccharptr(input_type.to_string())) {
        *lib = Some(lib_candidate.clone());
    }
}

fn check_for_error(error: typedefs::ConstCCharPtr) {
    if std::ptr::null() == error {
        return;
    }

    let e = match casting::ccharptr_to_string(error) {
        Ok(r) => r,
        Err(_) => String::from("Failed to decode an error message."),
    };

    handle_error(e);
}

fn handle_error<S: Into<String>>(error: S) {
    error!("{}", error.into());
    std::process::exit(-1);
}

fn main() {
    init::init_logger();
    let args = match CliArgs::new_from_args() {
        Ok(a) => a,
        Err(e) => return handle_error(format!("Failed to parse arguments: {}", e)),
    };
    let app_config_struct = app_config::create();

    let mut modules = load_modules().into_iter();
    let mut input_module: Option<Rc<LibModule>> = None;
    let mut output_module: Option<Rc<LibModule>> = None;
    while let Some(lm) = modules.next() {
        assign_lib(&mut input_module, &lm, args.input.get("type"));
        assign_lib(&mut output_module, &lm, args.output.get("type"));
    }

    let input_module = match input_module {
        None => return handle_error("Cannot find a module for input."),
        Some(m) => m,
    };
    // let output_module = match output_module {
    //     None => return Err(Error::new(ErrorKind::InvalidInput, "Cannot find a module for output.")),
    //     Some(m) => m,
    // };

    let (len, input_args_cchar_ptr) = args.input_as_c_char_ptr();
    let (handle, err) = (input_module.module.init_reader)(&app_config_struct as *const _, len, input_args_cchar_ptr as *const typedefs::ConstCCharPtr);
    check_for_error(err);

    let (result, err) = (input_module.module.read)(handle);
    check_for_error(err);
    let res = match casting::ccharptr_to_string(result) {
        Ok(r) => r,
        Err(_) => return handle_error("Failed to read from datasource."),
    };
    println!("{}", res);


    // if std::ptr::null() == err {
    //     let res = match casting::ccharptr_to_string(result) {
    //         Ok(r) => r,
    //         Err(_) => return Err(Error::new("Failed.")),
    //     };
    //     println!("{}", res);
    // } else {
    //     let e = match casting::ccharptr_to_string(err) {
    //         Ok(r) => r,
    //         Err(_) => String::from("Failed to decode an error message."),
    //     };
    //     return Err(Error::new(e));
    // }
}
