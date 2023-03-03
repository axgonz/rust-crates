use az_app_variables::*;

#[derive(New, Init, Debug)]
pub struct AppVariables {
    // The derive(Init) macro uses the property_name.to_uppercase()
    //  to find env variables when init() is called on an instance
    //  of AppVariables.  
    pub azure_keyvault_name: String,
    pub azure_storageaccount_name: String,
}

fn main() {
    /*
        Make sure to export the env variables that 
         are part of the struct definition:

         export AZURE_KEYVAULT_NAME=myKvName
         export AZURE_STORAGEACCOUNT_NAME=myStName
    */
    let mut app_variables = AppVariables::new();
    AppVariables::init(&mut app_variables);
    println!("\n{:#?}\n", app_variables);
}