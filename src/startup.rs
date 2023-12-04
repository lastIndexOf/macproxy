use hudsucker::{
    async_trait::async_trait,
    certificate_authority::OpensslAuthority,
    hyper::{Body, Request},
    openssl::{hash::MessageDigest, pkey::PKey, x509::X509},
    *,
};
use std::net::SocketAddr;
use tracing::{error, info};

use crate::{
    configuration::AppSettings, controller::handle_raycast_ai_chat, global_proxy::set_global_proxy,
};

async fn shutdown_signal(origin_proxy: AppSettings) {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");

    info!("Shutting down...");

    set_global_proxy(&origin_proxy).expect("revert global proxy failed");
}

#[derive(Clone)]
struct MacProxy;

// TODO 代理某些接口会直接报 443
// 1. icloud 相关接口
// 2. 苹果其他接口
// 3. 天气接口
// 4. iphone 模拟器打开后访问不了 google
#[async_trait]
impl HttpHandler for MacProxy {
    async fn handle_request(
        &mut self,
        _ctx: &HttpContext,
        req: Request<Body>,
    ) -> RequestOrResponse {
        let method = req.method();
        let url = req.uri().to_string();

        match (method, &url[..]) {
            (&hyper::Method::POST, "https://backend.raycast.com/api/v1/ai/chat_completions") => {
                handle_raycast_ai_chat(req).await.into()
            }
            _ => req.into(),
        }
    }
}

#[async_trait]
impl WebSocketHandler for MacProxy {}

pub async fn run(addr: &str, origin_proxy: AppSettings) -> anyhow::Result<()> {
    let private_key_bytes: &[u8] = include_bytes!("./ca/mac_proxy.key");
    let ca_cert_bytes: &[u8] = include_bytes!("./ca/mac_proxy.crt");
    let private_key =
        PKey::private_key_from_pem(private_key_bytes).expect("Failed to parse private key");
    let ca_cert = X509::from_pem(ca_cert_bytes).expect("Failed to parse CA certificate");

    let ca = OpensslAuthority::new(private_key, ca_cert, MessageDigest::sha256(), 1_000);

    let proxy = Proxy::builder()
        .with_addr(addr.parse::<SocketAddr>()?)
        .with_rustls_client()
        .with_ca(ca)
        .with_http_handler(MacProxy)
        .build();

    if let Err(e) = proxy.start(shutdown_signal(origin_proxy)).await {
        error!("unexpected error when proxy -- {}", e);
    }

    Ok(())
}
