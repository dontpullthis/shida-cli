use shida_core::ffi::app_config::{AppConfig, Functions, LogFunctions};

use crate::ffi::log;

pub fn create() -> AppConfig {
    AppConfig {
        functions: Functions {
            log: LogFunctions {
                debug: log::debug,
                error: log::error,
                info: log::info,
                warning: log::warning,
            }
        }
    }
}