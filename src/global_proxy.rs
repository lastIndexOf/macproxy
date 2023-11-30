use std::process::Command;

use tracing::info;

use crate::configuration::Settings;

pub fn set_global_proxy(settings: &Settings) -> anyhow::Result<()> {
    let host = format!("{}", settings.app.host);
    let port = format!("{}", settings.app.port);

    // Set Http proxy
    let _ = Command::new("networksetup")
        .arg("-setwebproxy")
        .arg("Wi-Fi")
        .arg(&host)
        .arg(&port)
        .output()?;

    // Set Https proxy
    let _ = Command::new("networksetup")
        .arg("-setsecurewebproxy")
        .arg("Wi-Fi")
        .arg(&host)
        .arg(&port)
        .output()?;

    info!("Http Proxy set to {}:{}", host, port);

    Ok(())
}
