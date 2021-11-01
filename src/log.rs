pub fn info(s: impl AsRef<str>) {
    println!("{:?} | {}", chrono::Local::now().to_rfc3339(), s.as_ref());
}

use info as log_info;

pub fn access(info: warp::log::Info) {
    log_info(format!(
        "{} {} {} {}ms {} {}",
        info.method().to_string(),
        info.path(),
        info.status().as_u16(),
        info.elapsed().as_millis(),
        info.remote_addr()
            .map(|a| a.ip().to_string())
            .unwrap_or(String::new()),
        info.user_agent().unwrap_or("")
    ));
}
