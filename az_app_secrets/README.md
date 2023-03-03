# az_app_secrets

## Cargo.toml
``` toml
...
[dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
az_app_identity = { version = "0.1.0", git ="https://github.com/axgonz/rust-crates" }
az_app_variables = { version = "0.1.0", git ="https://github.com/axgonz/rust-crates" }
az_app_secrets = { version = "0.1.0", git ="https://github.com/axgonz/rust-crates" }
...
```

## main.rs
``` rust
use az_app_identity::*;
use az_app_variables::*;
use az_app_secrets::*;

#[derive(AzAppVariablesNew, AzAppVariablesInit, Debug)]
pub struct AppVariables {
    pub azure_keyvault_name: String,
    pub azure_storageaccount_name: String,
}

#[derive(AzAppSecretsNew, AzAppSecretsInit, Debug)]
pub struct AppSecrets {
    pub azure_storageaccount_key: String,
}

#[tokio::main]
async fn main() {
    let app_identity: Arc<DefaultAzureCredential> = AppIdentity::new();

    let mut app_variables = AppVariables::new();
    AppVariables::init(&mut app_variables);
    println!("\n{:#?}\n", app_variables);

    let mut app_secrets = AppSecrets::new();
    AppSecrets::init(&mut app_secrets, &app_variables.azure_keyvault_name, app_identity).await;
    println!("\n{:#?}\n", app_secrets);
}
```