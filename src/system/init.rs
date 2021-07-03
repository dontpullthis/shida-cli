use env_logger::Env;

pub fn init_logger() {
    let env = Env::new().filter("LOG_LEVEL").write_style("LOG_STYLE");
    match env_logger::try_init_from_env(env) {
        Ok(_) => {},
        Err(_) => panic!("Failed to init logger."),
    }
}