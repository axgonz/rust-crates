# az_app_variables

## Cargo.toml
``` toml
...
[dependencies]
az_app_variables = { version = "0.1.0", git ="https://github.com/axgonz/rust-crates" }
...
```

## main.rs
``` rust
use az_app_variables::*;

#[derive(AzAppVariablesNew, AzAppVariablesInit, Debug)]
pub struct AppVariables {
    pub azure_keyvault_name: String,
    pub azure_storageaccount_name: String,
}

fn main() {
    let mut app_variables = AppVariables::new();
    AppVariables::init(&mut app_variables);
    println!("\n{:#?}\n", app_variables);
}
```