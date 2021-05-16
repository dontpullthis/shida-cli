mod system;

use crate::system::cli_args::CliArgs;

fn main() {
    let args = match CliArgs::new_from_args() {
        Ok(a) => a,
        Err(_) => return,
    };

    for (k, v) in args.input {
        println!("Input: {}={}", k, v);
    }
    for (k, v) in args.output {
        println!("Output: {}={}", k, v);
    }
    println!("End");
}
