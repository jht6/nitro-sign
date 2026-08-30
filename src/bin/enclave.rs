use nitro_sign::config::Config;
use nitro_sign::enclave;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config = Config::from_env();

    if let Err(e) = enclave::serve(config.vsock_port).await {
        tracing::error!("enclave server error: {e}");
        std::process::exit(1);
    }
}
