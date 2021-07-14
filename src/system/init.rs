use env_logger::Env;
use env_logger::fmt::Color;
use std::io::Write;

pub fn init_logger() {
    let env = Env::new().filter("LOG_LEVEL").write_style("LOG_STYLE");
    env_logger::builder()
        .parse_env(env)
        .format(|buf, record| {
            let mut level_style = buf.style();
            level_style.set_color(Color::Blue).set_bold(true);

            writeln!(buf, "[{} {}] {}", buf.timestamp(), level_style.value(record.level()), record.args())
        })
        .init();
}