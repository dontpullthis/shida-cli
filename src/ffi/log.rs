use shida_core::ffi::casting;
use shida_core::ffi::typedefs;

use log::{debug, error, info, warn};

pub fn debug(message: typedefs::ConstCCharPtr) {
    match casting::ccharptr_to_string(message) {
        Ok(m) => debug!("{}", m),
        Err(_) => error!("Failed to cast a log message from C char pointer to string")
    }
}

pub fn error(message: typedefs::ConstCCharPtr) {
    match casting::ccharptr_to_string(message) {
        Ok(m) => error!("{}", m),
        Err(_) => error!("Failed to cast a log message from C char pointer to string")
    }
}

pub fn info(message: typedefs::ConstCCharPtr) {
    match casting::ccharptr_to_string(message) {
        Ok(m) => info!("{}", m),
        Err(_) => error!("Failed to cast a log message from C char pointer to string")
    }
}

pub fn warning(message: typedefs::ConstCCharPtr) {
    match casting::ccharptr_to_string(message) {
        Ok(m) => warn!("{}", m),
        Err(_) => error!("Failed to cast a log message from C char pointer to string")
    }
}