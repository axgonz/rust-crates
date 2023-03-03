use std::sync::Arc;
use azure_identity::DefaultAzureCredential;

use azure_security_keyvault::SecretClient;
use async_trait::async_trait;

pub trait AzAppSecretsNew {
    fn new() -> Self;
}

#[async_trait]
pub trait GetFromKeyVault {
    async fn get_from_key_vault(name: &str, vault_name: &str, credential: Arc<DefaultAzureCredential>) -> String {
        let vault_uri = format!("https://{}.vault.azure.net", vault_name);
        let secret_client = SecretClient::new(&vault_uri, credential).unwrap();
        match secret_client.get(name).await {
            Ok(value) => {
                println!("[az_app_secrets] Requesting key vault secret {:#?}...Ok", name);
                return value.value
            }
            Err(error) => {
                println!("[az_app_secrets] Requesting key vault secret {:#?}...Err", name);
                panic!("{}", error)
            }
        }
    }
}