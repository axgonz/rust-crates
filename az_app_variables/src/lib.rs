pub use app_variables_new_macro::New;
use app_variables_new_macro_derive::New;

pub use app_variables_init_macro::Init;
use app_variables_init_macro_derive::Init;

use std::{
    env,
};

#[derive(New, Init, Debug)]
pub struct AppVariables {
    // The derive(Init) macro uses the property_name.to_uppercase()
    //  to find env variables when init() is called on an instance
    //  of AppVariables.  
    pub azure_storageaccount_name: String,
}

impl AppVariables {
    pub fn get_from_env(name: &str) -> String {
        match env::var(name) {
            Ok(value) => return value,
            Err(_) => {
                panic!("Unable to load environment variable: {:#?}", name)
            }
        }
    }
    pub fn get_from_cfg(_name: &str) -> Result<String, String> {
    //     let cfg = confy::load("my-app-name").unwrap();

    //     // ...
        
    //     match env::var(name) {
    //         Ok(value) => return Ok(value),
    //         Err(_) => {
    //             return Err(format!("Unable to load configuration variable: {:#?}", name));
    //         }
    //     }
        return Err("NOT_IMPLEMENTED".to_string())
    }
}