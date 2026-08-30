use nitro_sign::config::Config;
use nitro_sign::host;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config = Config::from_env();

    if let Err(e) = host::serve(
        &config.host_bind_addr,
        config.enclave_cid,
        config.vsock_port,
    )
    .await
    {
        tracing::error!("host server error: {e}");
        std::process::exit(1);
    }
}
