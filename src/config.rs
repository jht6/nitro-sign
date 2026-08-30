//! 共享运行配置：host 与 enclave 都从这里读取参数。
//!
//! enclave 的 CID 由 Nitro Enclaves 运行时分配（`nitro-cli run-enclave` 返回），
//! 本项目约定使用 `10`。所有值都可通过环境变量覆盖，未设置时回退到默认值。

use std::env;

/// 默认 enclave CID。
///
/// 该值需要与 `nitro-cli run-enclave` 实际分配的 CID 保持一致，
/// 可通过 `NITRO_ENCLAVE_CID` 环境变量覆盖。
pub const DEFAULT_ENCLAVE_CID: u32 = 10;

/// host 与 enclave 约定的 vsock 端口。
pub const DEFAULT_VSOCK_PORT: u32 = 5005;

/// host HTTP server 对外监听的地址。
pub const DEFAULT_HOST_BIND_ADDR: &str = "0.0.0.0:8080";

/// 运行配置。
#[derive(Debug, Clone)]
pub struct Config {
    /// enclave 的 vsock CID。
    pub enclave_cid: u32,
    /// vsock 通信端口（host 与 enclave 必须一致）。
    pub vsock_port: u32,
    /// host HTTP server 监听地址。
    pub host_bind_addr: String,
}

impl Config {
    /// 从环境变量加载配置，未设置时使用默认值。
    pub fn from_env() -> Self {
        Self {
            enclave_cid: env::var("NITRO_ENCLAVE_CID")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(DEFAULT_ENCLAVE_CID),
            vsock_port: env::var("NITRO_VSOCK_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(DEFAULT_VSOCK_PORT),
            host_bind_addr: env::var("NITRO_HOST_BIND_ADDR")
                .unwrap_or_else(|_| DEFAULT_HOST_BIND_ADDR.to_string()),
        }
    }
}
