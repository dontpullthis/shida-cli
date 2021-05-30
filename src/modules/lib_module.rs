
use libloading::Library;

use shida_core::module::Module;

#[allow(dead_code)]
pub struct LibModule {
    lib: Library,
    pub module: Module,
}

impl LibModule {
    pub fn new(lib: Library, module: Module) -> LibModule {
        LibModule {
            lib,
            module
        }
    }
}
