use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use tracing::debug;

#[get("/{url:.*}")]
async fn proxy_request(req: HttpRequest) -> Result<HttpResponse, Error> {
    debug!("request = {req:#?}");

    Ok(HttpResponse::Ok().finish())
}
