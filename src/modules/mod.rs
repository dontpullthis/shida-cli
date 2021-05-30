pub mod lib_module;

use std::collections::LinkedList;
use std::fs;
use std::rc::Rc;

use libloading::{Library, Symbol};

use shida_core::module::Module;

use crate::modules::lib_module::LibModule;

fn load_module_from_lib(path: String) -> Result<LibModule, Box<dyn std::error::Error>> {
    unsafe {
        let lib = Library::new(path)?;
        let load_func: Symbol<unsafe extern fn() -> Module> = lib.get(b"load")?;
        let loaded_module = load_func();
        Ok(LibModule::new(lib, loaded_module))
    }
}

pub fn load_modules() -> LinkedList<Rc<LibModule>> {
    let mut result: LinkedList<Rc<LibModule>> = LinkedList::new();

    let paths = fs::read_dir("./.dev/modules").unwrap(); // TODO: replace with configurable path
    for path_item in paths {
        let path_dir_entry = match path_item {
            Ok(p) => p,
            Err(_) => continue,
        };
        let lib_path = path_dir_entry.path().into_os_string().into_string().unwrap();
        match load_module_from_lib(lib_path) {
            Ok(lm) => {
                result.push_back(Rc::from(lm));
            },
            Err(_) => println!("Error"),
        };
    }

    result
}