//! enclave 侧的 HTTP server。
//!
//! Nitro Enclave 没有 TCP/IP 网络栈，只能通过 vsock 与宿主（父实例）通信。
//! 因此这里的 axum server 直接监听 vsock socket：`tokio-vsock` 的
//! `VsockListener`（启用 `axum08` feature 后）实现了 `axum::serve::Listener`。

use axum::{Json, Router, routing::get};
use serde::Serialize;
use tokio_vsock::{VMADDR_CID_ANY, VsockAddr, VsockListener};

/// `/demo` 的响应体。
#[derive(Debug, Serialize)]
struct DemoResponse {
    data: &'static str,
}

/// GET /demo → {"data":"enclave"}
async fn demo() -> Json<DemoResponse> {
    Json(DemoResponse { data: "enclave" })
}

/// 构造 enclave 的路由。
pub fn app() -> Router {
    Router::new().route("/demo", get(demo))
}

/// 在 vsock 端口 `port` 上启动 enclave HTTP server。
pub async fn serve(port: u32) -> std::io::Result<()> {
    // enclave 内监听任意 CID（宿主会以分配给 enclave 的 CID 连进来）。
    let listener = VsockListener::bind(VsockAddr::new(VMADDR_CID_ANY, port))?;

    tracing::info!("enclave http server listening on vsock port {port}");
    axum::serve(listener, app()).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn demo_returns_enclave_json() {
        let resp = app()
            .oneshot(Request::builder().uri("/demo").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get("content-type").unwrap(),
            "application/json"
        );

        let body = resp.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], br#"{"data":"enclave"}"#);
    }
}
