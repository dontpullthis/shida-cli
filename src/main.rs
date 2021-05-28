mod system;

use crate::system::cli_args::CliArgs;

use shida_core::ffi::string_to_ccharptr;
use shida_core::module::{LibModule, load_modules};



fn main() {
    let args = match CliArgs::new_from_args() {
        Ok(a) => a,
        Err(_) => return,
    };

    // TODO: DELETEME
    for (k, v) in &args.input {
        println!("Input: {}={}", k, v);
    }
    for (k, v) in &args.output {
        println!("Output: {}={}", k, v);
    }

    let mut modules = load_modules().into_iter();
    // TODO: implement assignment of modules. Maybe use reference counters?
    // let input_module: Option<LibModule> = None;
    // let output_module: Option<LibModule> = None;
    while let Some(lm) = modules.next() {
        let can_handle_fn = lm.module.can_handle;
        unsafe {
            let input_type = match args.input.get("type") {
                Some(i) => i,
                None => continue,
            };
            println!("Can parse: {}", can_handle_fn(string_to_ccharptr(input_type.to_string())))
        }
        
    }

    println!("End");
}
