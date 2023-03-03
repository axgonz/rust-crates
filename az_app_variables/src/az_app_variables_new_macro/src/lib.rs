use std::env;

pub trait AzAppVariablesNew {
    fn new() -> Self;
}

pub trait AzAppVariablesGetFromEnv {
    fn get_from_env(name: &str) -> String {
        match env::var(name) {
            Ok(value) => return value,
            Err(_) => {
                panic!("Unable to load environment variable: {:#?}", name)
            }
        }
    }
}