use tracing::{dispatcher::set_global_default, Subscriber};
use tracing_appender::{non_blocking, rolling};
// use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt::MakeWriter, prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Registry,
};

use crate::configuration::Settings;

pub fn get_subscriber<W>(
    env_filter: &str,
    settings: &Settings,
    writer: W,
) -> impl Subscriber + Send + Sync
where
    W: for<'writer> MakeWriter<'writer> + Clone + Send + Sync + 'static,
{
    let env_layer =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    // let fmt_layer = BunyanFormattingLayer::new("mac_proxy".into(), writer.clone());

    let log_dir = settings.log_dir.clone();
    let log_file_prefix = "mac_proxy.log_";

    // TODO: 只创建文件没有日志写入
    let appender = rolling::hourly(log_dir, log_file_prefix);
    let (non_blocking_appender, _guard) = non_blocking(appender);

    // create_retention_timer(settings.log_dir.clone());

    Registry::default()
        .with(env_layer)
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking_appender))
        .with(tracing_subscriber::fmt::layer().with_writer(writer.clone()))
    // .with(fmt_layer)
    // .with(JsonStorageLayer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to init tracing log");
    set_global_default(subscriber.into()).expect("Failed to set subscriber");
}

// 设置定时器，只保留 7 天的日志
// fn create_retention_timer(log_dir: String) {
//     let log_dir = std::path::Path::new(&log_dir);

//     loop {
//         if !log_dir.exists() {
//             continue;
//         }

//         if let Ok(entries) = std::fs::read_dir(log_dir) {
//             for entry in entries {
//                 if let Ok(entry) = entry {
//                     if let Ok(metadata) = std::fs::metadata(entry.path()) {
//                         if let Ok(time) = metadata.modified() {
//                             let now = std::time::SystemTime::now();
//                             let seven_days = std::time::Duration::from_secs(60 * 60 * 24 * 7);

//                             if let Ok(elapsed) = time.elapsed() {
//                                 if elapsed > seven_days {
//                                     std::fs::remove_file(entry.path()).unwrap();
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }

//         std::thread::sleep(std::time::Duration::from_secs(6 * 6 * 10));
//     }
// }
