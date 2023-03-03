# az_app_identity

## Cargo.toml
``` toml
...
[dependencies]
az_app_identity = { version = "0.1.0", git ="https://github.com/axgonz/rust-crates" }
...
```

## main.rs
``` rust
use az_app_identity::*;

fn main() {
    let _app_identity: Arc<DefaultAzureCredential> = AppIdentity::new();
}
```