use std::collections::HashMap;

use clap::{Arg, App};

use shida_core::ffi;

pub struct CliArgs {
    pub input: HashMap<String, String>,
    pub output: HashMap<String, String>,
}

impl CliArgs {
    pub fn new() -> CliArgs {
        CliArgs {
            input: HashMap::new(),
            output: HashMap::new(),
        }
    }

    pub fn new_from_args() -> Result<CliArgs, String> {
        let matches = App::new("Shida")
            .version("0.1")
            .about("A data migrator")
            .arg(Arg::with_name("src-param")
                .short("s")
                .long("src-param")
                .value_name("SRC_PARAM")
                .help("A parameter for source connection")
                .multiple(true)
                .takes_value(true))
            .arg(Arg::with_name("dest-param")
                .short("d")
                .long("dest-param")
                .value_name("SRC_PARAM")
                .help("A parameter for destination connection")
                .multiple(true)
                .takes_value(true))
            .get_matches();

        let mut args = CliArgs::new();

        args.input = extract_params(&matches, "src-param");
        args.output = extract_params(&matches, "dest-param");


        Ok(args)
    }

    pub fn input_as_byte_vec(&self) -> Vec<Vec<u8>> {
        let mut result: Vec<Vec<u8>> = Vec::new();
        for (k, v) in &self.input {
            let kv = vec![k.as_str(), v.as_str()];
            let kv = kv.join("=");
            result.push(Vec::from(kv.as_bytes()));
        }
        result
    }

    pub fn input_as_c_char_ptr(&self) -> (usize, *mut ffi::MutCCharPtr) {
        let size = self.input.len();
        let result = unsafe { ffi::malloc::<ffi::MutCCharPtr>(size) };
        let input_byte_vec = self.input_as_byte_vec();
        for (index, line) in input_byte_vec.iter().enumerate() {
            unsafe {
                let val = result.offset(index as isize);
                *val = ffi::bytes_vec_to_ccharptr(line);
            }
        }
        (size, result)
    }
}

fn extract_params(matches: &clap::ArgMatches, arg_name: &str) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    let mut args = match matches.values_of(arg_name) {
        None => return result,
        Some(cli_args) => cli_args,
    };

    while let Some(param) = args.next() {
        let pos = match param.find("=") {
            Some(position) => position,
            None => continue,
        };
        let key = String::from(&param[..pos]);
        let value = String::from(&param[pos+1..]);
        result.insert(key, value);
    }

    result
}