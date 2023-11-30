use mac_proxy::{
    configuration::get_configuration,
    global_proxy::set_global_proxy,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use tracing::info;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let settings = get_configuration()?;

    let subscriber = get_subscriber("info", &settings, std::io::stdout);
    init_subscriber(subscriber);

    set_global_proxy(&settings)?;

    info!(
        "server running at http://{}:{}",
        settings.app.host, settings.app.port
    );

    Ok(run(&format!("{}:{}", settings.app.host, settings.app.port)).await?)
}
