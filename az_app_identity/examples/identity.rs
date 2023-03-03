use az_app_identity::*;

fn main() {
    let _app_identity: Arc<DefaultAzureCredential> = AppIdentity::new();
}