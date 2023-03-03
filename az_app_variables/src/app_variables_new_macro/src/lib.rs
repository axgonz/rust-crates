use std::env;

pub trait New {
    fn new() -> Self;
}

pub trait GetFromEnv {
    pub fn get_from_env(name: &str) -> String {
        match env::var(name) {
            Ok(value) => return value,
            Err(_) => {
                panic!("Unable to load environment variable: {:#?}", name)
            }
        }
    }
}