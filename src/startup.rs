use std::net::TcpListener;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};

use crate::routes::{health_check, proxy_request};

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            // .wrap(TracingLogger::default())
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .service(proxy_request)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
