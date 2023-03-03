use std::env;

pub trait AzAppVariablesNew {
    fn new() -> Self;
}

pub trait AzAppVariablesGetFromEnv {
    fn get_from_env(name: &str) -> String {
        match env::var(name) {
            Ok(value) => {
                println!("[az_app_variables] Reading environment variable {:#?}...Ok", name);
                return value
            }
            Err(error) => {
                println!("[az_app_variables] Reading environment variable {:#?}...Err", name);
                panic!("{}", error)
            }
        }
    }
}