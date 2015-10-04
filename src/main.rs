extern crate lish;

#[cfg(not(test))]
use lish::repl::Repl;

#[cfg(not(test))]
use lish::config::Config;

#[cfg(not(test))]
use lish::config::Error as ConfigError;

#[cfg(not(test))]
fn main() {
    let config = match Config::new() {
        Ok(config) => config,
        Err(ConfigError::InvalidHome) => {
            panic!("lish: can't determine user home");
        },
    };

    let repl = Repl::new(config);

    repl.run();
}
