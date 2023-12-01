use std::process::Command;

use tracing::info;

use crate::configuration::AppSettings;

pub fn set_global_proxy(settings: &AppSettings) -> anyhow::Result<AppSettings> {
    let origin_proxy = get_global_proxy()?;

    let host = format!("{}", settings.host);
    let port = format!("{}", settings.port);

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

    Ok(origin_proxy)
}

fn get_global_proxy() -> anyhow::Result<AppSettings> {
    let host = std::process::Command::new("sh")
        .arg("-c")
        .arg("networksetup -getwebproxy Wi-Fi | grep 'Server:' | awk '{print $2}'")
        .output()
        .unwrap();

    let host = String::from_utf8(host.stdout).unwrap();
    let host = host.trim();

    let port = std::process::Command::new("sh")
        .arg("-c")
        .arg("networksetup -getwebproxy Wi-Fi | grep 'Port:' | awk '{print $2}'")
        .output()
        .unwrap();

    let port = String::from_utf8(port.stdout).unwrap();
    let port = port.trim();

    Ok(AppSettings {
        host: host.to_string(),
        port: port.parse()?,
    })
}

#[cfg(test)]
mod test_networksetup {
    #[test]
    fn test_current_proxy_settings() {
        let host = std::process::Command::new("sh")
            .arg("-c")
            .arg("networksetup -getwebproxy Wi-Fi | grep 'Server:' | awk '{print $2}'")
            .output()
            .unwrap();

        let host = String::from_utf8(host.stdout).unwrap();
        let host = host.trim();

        let port = std::process::Command::new("sh")
            .arg("-c")
            .arg("networksetup -getwebproxy Wi-Fi | grep 'Port:' | awk '{print $2}'")
            .output()
            .unwrap();

        let port = String::from_utf8(port.stdout).unwrap();
        let port = port.trim();

        assert_eq!("127.0.0.1", host);
        assert_eq!("12506", port);
    }
}
