use stackdriver_logger::Service;
use std::convert::Infallible;
use warp::Filter;
use yukikaze::{send_message, Message};

async fn handle(msg: Message) -> Result<&'static str, Infallible> {
    send_message(msg).await;
    log::info!("handle done");
    Ok("done")
}

fn main() {
    let service = Service {
        name: std::env!("CARGO_PKG_NAME").to_string(),
        version: std::env!("CARGO_PKG_VERSION").to_string(),
    };
    stackdriver_logger::init_with(service.into(), true);
    log_panics::init();
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|port| port.parse().ok())
        .unwrap_or(8080);
    let server = warp::post().and(warp::body::json()).and_then(handle);
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(warp::serve(server).run(([0, 0, 0, 0], port)));
}
