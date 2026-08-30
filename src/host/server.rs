//! host 侧的 HTTP server。
//!
//! 对外暴露 TCP HTTP 接口；`/demo` 收到请求后通过 vsock 转发给
//! enclave，并把 enclave 的响应（状态码 + body）原样返回。

use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;

use super::enclave_client::EnclaveClient;

/// host server 持有的共享状态。
#[derive(Clone)]
struct AppState {
    enclave: EnclaveClient,
}

/// GET /demo → 透传 enclave 的响应。
async fn demo(State(state): State<AppState>) -> Response {
    match state.enclave.get_demo().await {
        Ok(resp) => resp,
        Err(e) => {
            tracing::error!("failed to reach enclave: {e}");
            (StatusCode::BAD_GATEWAY, format!("enclave unreachable: {e}")).into_response()
        }
    }
}

/// 启动 host HTTP server。
pub async fn serve(host_bind_addr: &str, enclave_cid: u32, vsock_port: u32) -> std::io::Result<()> {
    let state = AppState {
        enclave: EnclaveClient::new(enclave_cid, vsock_port),
    };

    let app = Router::new().route("/demo", get(demo)).with_state(state);

    let listener = tokio::net::TcpListener::bind(host_bind_addr).await?;
    tracing::info!("host http server listening on {host_bind_addr}");
    axum::serve(listener, app).await
}
