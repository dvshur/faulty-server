mod config;
mod dto;
mod log;
mod strategy;

use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = envy::from_env::<config::Config>().unwrap();

    log::info(format!("Starting service on port {:?}", &config.port));

    let routes = strategy::constant_time::route();
    // .or(strategy::???::route());

    let filters = routes.with(warp::log::custom(log::access));

    warp::serve(filters).run(([0, 0, 0, 0], config.port)).await;
    Ok(())
}
