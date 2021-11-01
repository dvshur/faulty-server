use std::convert::Infallible;

use warp::Filter;

mod config {
    use crate::config::from_millis;
    use serde::Deserialize;
    use tokio::time::Duration;

    fn default_response_time() -> Duration {
        Duration::from_secs(1)
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct Config {
        #[serde(
            default = "default_response_time",
            deserialize_with = "from_millis",
            rename = "response_time_millis"
        )]
        pub response_time: Duration,
    }
}

pub fn route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let response_time = envy::prefixed("CONSTANT_TIME__")
        .from_env::<config::Config>()
        .unwrap()
        .response_time;

    warp::path!("constant_time").and_then(move || async move {
        tokio::time::sleep(response_time).await;
        let ok = crate::dto::ok();
        let reply = warp::reply::json(&ok);
        Ok::<_, Infallible>(reply)
    })
}
