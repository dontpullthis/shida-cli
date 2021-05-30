mod modules;
mod system;

use std::io::{Error, ErrorKind};
use std::rc::Rc;

use crate::modules::lib_module::LibModule;
use crate::modules::load_modules;
use crate::system::cli_args::CliArgs;

use shida_core::ffi::string_to_ccharptr;


fn assign_lib(lib: &mut Option<Rc<LibModule>>, lib_candidate: &Rc<LibModule>, lib_name_arg: Option<&String>) {
    let can_handle_fn = lib_candidate.module.can_handle;
    let input_type = match lib_name_arg {
        Some(i) => i,
        None => return,
    };
    unsafe {
        if can_handle_fn(string_to_ccharptr(input_type.to_string())) {
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

    if input_module.is_none() {
        return Err(Error::new(ErrorKind::InvalidInput, "Cannot find a module for input."))
    }
    if output_module.is_none() {
        return Err(Error::new(ErrorKind::InvalidInput, "Cannot find a module for output."))
    }

    Ok(())
}
