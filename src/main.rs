use mac_proxy::{
    configuration::get_configuration,
    global_proxy::set_global_proxy,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use tracing::info;

#[tokio::main(flavor = "multi_thread", worker_threads = 12)]
async fn main() -> anyhow::Result<()> {
    let settings = get_configuration()?;

    let (subscriber, _guard) = get_subscriber("info", &settings, std::io::stdout);
    init_subscriber(subscriber);

    let origin_proxy = set_global_proxy(&settings.app)?;

    info!(
        "server running at http://{}:{}",
        settings.app.host, settings.app.port
    );

    Ok(run(
        &format!("{}:{}", settings.app.host, settings.app.port),
        origin_proxy,
    )
    .await?)
}
