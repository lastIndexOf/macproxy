use futures::StreamExt;
use hudsucker::{
    async_trait::async_trait,
    certificate_authority::OpensslAuthority,
    hyper::{Body, Request, Response},
    openssl::{hash::MessageDigest, pkey::PKey, x509::X509},
    tokio_tungstenite::tungstenite::Message,
    *,
};
use serde_json::Value;
use std::net::SocketAddr;
use tracing::{error, info};

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

#[derive(Clone)]
struct LogHandler;

#[async_trait]
impl HttpHandler for LogHandler {
    async fn handle_request(
        &mut self,
        _ctx: &HttpContext,
        req: Request<Body>,
    ) -> RequestOrResponse {
        let url = req.uri().to_string();

        match &url[..] {
            "https://backend.raycast.com/api/v1/ai/chat_completions" => {
                let (req, mut body) = req.into_parts();

                let mut payload: Value =
                    serde_json::from_slice(&hyper::body::to_bytes(body).await.unwrap()).unwrap();
                payload["model"] = Value::String(String::from("gpt-4-1106-preview"));

                body = serde_json::to_vec(&payload).unwrap().into();

                return Request::from_parts(req, body).into();
            }
            _ => {}
        }

        req.into()
    }
}

#[async_trait]
impl WebSocketHandler for LogHandler {
    async fn handle_message(&mut self, _ctx: &WebSocketContext, msg: Message) -> Option<Message> {
        Some(msg)
    }
}

pub async fn run(addr: &str) -> anyhow::Result<()> {
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
        .with_http_handler(LogHandler)
        .build();

    if let Err(e) = proxy.start(shutdown_signal()).await {
        error!("{}", e);
    }

    Ok(())
}
