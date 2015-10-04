use std;

#[derive(Debug)]
pub enum Error {
    InvalidHome,
}

pub struct Config {
    pub init_file: String,
}

impl Config {
    pub fn new() -> Result<Config, Error> {
        Ok(Config{
            init_file: try!(read_init_file()),
        })
    }
}

fn read_init_file() -> Result<String, Error> {
    let mut home = match std::env::home_dir() {
        Some(dir) => dir,
        None => return Err(Error::InvalidHome),
    };

    home.push(".init.lish");

    Ok(home.as_path().to_str().unwrap().to_string())
}
