use actix_web::{get, HttpRequest, HttpResponse, Responder};
use tracing::{debug, error};

#[get("/{url:.*}")]
async fn proxy_request(req: HttpRequest) -> impl Responder {
    match do_proxy_request(req).await {
        Ok(response) => response,
        Err(e) => {
            error!("proxy request error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn do_proxy_request(req: HttpRequest) -> anyhow::Result<HttpResponse> {
    let url = req.uri();
    let method = req.method();
    let headers = req.headers();

    debug!(
        "request = {url:#?} method = {method:#?}",
        url = url,
        method = method
    );

    let client = reqwest::Client::new();
    for (key, value) in headers {
        let _ = client.head(format!("{}: {}", key.to_string(), value.to_str()?));
    }

    let res = client
        .request(method.clone(), url.to_string())
        .send()
        .await?
        .bytes_stream();

    Ok(HttpResponse::Ok().streaming(res))
}
