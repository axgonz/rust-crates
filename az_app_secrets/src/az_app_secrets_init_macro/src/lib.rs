use std::sync::Arc;
use azure_identity::DefaultAzureCredential;

use async_trait::async_trait;

#[async_trait]
pub trait AzAppSecretsInit {
    async fn init(me: &mut Self, vault_name: &str, credential: Arc<DefaultAzureCredential>);
}