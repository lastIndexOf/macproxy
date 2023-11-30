use hudsucker::hyper::{self, Body, Request, Response, StatusCode};
use serde_json::Value;
use tracing::{error, info};

pub async fn handle_raycast_ai_chat(req: Request<Body>) -> Response<Body> {
    match do_handle_raycast_ai_chat(req).await {
        Ok(response) => response.into(),
        Err(err) => {
            error!("Failed to proxy raycast ai chat -- {}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()
                .into()
        }
    }
}

pub async fn do_handle_raycast_ai_chat(req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let (req, mut body) = req.into_parts();

    let bytes = hyper::body::to_bytes(body).await?;
    let mut content_len = bytes.len();
    let mut payload: Value = serde_json::from_slice(&bytes)?;

    body = if payload["source"] == "ai_chat" {
        info!("proxy raycast ai chat model to gpt-4-turbo");

        payload["model"] = Value::String(String::from("gpt-4-1106-preview"));

        let bytes = serde_json::to_vec(&payload)?;
        content_len = bytes.len();
        body = bytes.into();
        body
    } else {
        bytes.into()
    };

    let mut client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all("http://127.0.0.1:7890")?)
        .build()?
        .post(req.uri.to_string());

    for (key, value) in &req.headers {
        client = client.header(key.to_string(), value.to_str()?);
    }

    let res = client
        .header("Content-Length", content_len)
        .body(body)
        .send()
        .await?;

    Ok(Response::builder()
        .status(res.status())
        .body(Body::wrap_stream(res.bytes_stream()))?)
}

#[cfg(test)]
mod test_raycast_api {
    use serde_json::json;

    #[test]
    fn test_request_raycast_chat_api() {
        tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .unwrap()
            .block_on(async {
                let res = reqwest::Client::new()
                    .post("https://backend.raycast.com/api/v1/ai/chat_completions")
                    .json(&json!({
                        "debug": false,
                        "locale": "en-CN",
                        "messages": [
                            {
                                "author": "user",
                                "content": {
                                    "text": "你知道 stable diffusion 吗",
                                },
                            },
                        ],
                        "model": "gpt-4-1106-preview",
                        "provider": "openai",
                        "source": "ai_chat",
                        "system_instruction": "markdown",
                        "tools": [
                            "google",
                            "wikipedia",
                        ],
                    }))
                    .send()
                    .await
                    .unwrap();

                assert_eq!(200, res.status());
            });
    }
}
