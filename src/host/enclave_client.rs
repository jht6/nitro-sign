//! host → enclave 的 vsock HTTP 客户端。
//!
//! 通过 vsock 连接 enclave，在连接上跑 HTTP/1.1，把 enclave 的响应
//! 原样（状态码 + 响应头 + body）透传给调用方。

use axum::body::Body;
use axum::http::header::{self, HeaderName};
use axum::http::{Request, Response};
use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
use hyper_util::rt::TokioIo;
use tokio_vsock::{VsockAddr, VsockStream};

/// enclave 客户端：持有 enclave 的 CID 与 vsock 端口。
#[derive(Debug, Clone)]
pub struct EnclaveClient {
    cid: u32,
    port: u32,
}

impl EnclaveClient {
    pub fn new(cid: u32, port: u32) -> Self {
        Self { cid, port }
    }

    /// 请求 enclave 的 GET /demo，返回透传后的 axum 响应。
    pub async fn get_demo(&self) -> Result<Response<Body>, String> {
        let stream = VsockStream::connect(VsockAddr::new(self.cid, self.port))
            .await
            .map_err(|e| format!("vsock connect to {}:{} failed: {e}", self.cid, self.port))?;

        let io = TokioIo::new(stream);
        let (mut sender, conn) = hyper::client::conn::http1::handshake::<_, Empty<Bytes>>(io)
            .await
            .map_err(|e| format!("http handshake failed: {e}"))?;

        // 在后台驱动连接，直到请求完成。
        tokio::spawn(async move {
            let _ = conn.await;
        });

        let req = Request::builder()
            .uri(format!("http://enclave:{}/demo", self.port))
            .body(Empty::<Bytes>::new())
            .map_err(|e| format!("build request failed: {e}"))?;

        let resp = sender
            .send_request(req)
            .await
            .map_err(|e| format!("send request failed: {e}"))?;

        let status = resp.status();
        let headers = resp.headers().clone();
        let body = resp
            .into_body()
            .collect()
            .await
            .map_err(|e| format!("read response body failed: {e}"))?
            .to_bytes();

        // 透传状态码 + 端到端响应头 + body。
        // 过滤 hop-by-hop 头与 content-length（由 axum 重新计算）。
        let mut response = Response::new(Body::from(body));
        *response.status_mut() = status;
        for (name, value) in &headers {
            if name == header::CONTENT_LENGTH || is_hop_by_hop(name) {
                continue;
            }
            response.headers_mut().insert(name.clone(), value.clone());
        }

        Ok(response)
    }
}

/// 判断是否为 hop-by-hop 响应头（不应透传）。
fn is_hop_by_hop(name: &HeaderName) -> bool {
    matches!(
        name.as_str(),
        "connection"
            | "keep-alive"
            | "proxy-authenticate"
            | "proxy-authorization"
            | "te"
            | "trailer"
            | "transfer-encoding"
            | "upgrade"
    )
}
