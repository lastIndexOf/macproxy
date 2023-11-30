use std::process::Command;

use tracing::info;

use crate::configuration::Settings;

pub fn set_global_proxy(settings: &Settings) -> anyhow::Result<()> {
    let proxy = format!("http://{}:{}", settings.app.host, settings.app.port);

    // Set Http proxy
    let _ = Command::new("networksetup")
        .arg("-setwebproxy")
        .arg("Wi-Fi")
        .arg(&proxy)
        .output()?;

    info!("Http Proxy set to {}", proxy);

    // Set Https proxy
    let _ = Command::new("networksetup")
        .arg("-setsecurewebproxy")
        .arg("Wi-Fi")
        .arg(&proxy)
        .output()?;

    info!("Https Proxy set to {}", proxy);

    Ok(())
}
