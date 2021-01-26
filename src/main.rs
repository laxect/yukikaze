use warp::Filter;
use yukikaze::{send_message, Message};

fn handle(msg: Message) -> &'static str {
    tokio::task::spawn(send_message(msg));
    "done"
}

fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|port| port.parse().ok())
        .unwrap_or(8080);
    let server = warp::post().and(warp::body::json()).map(handle);
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(warp::serve(server).run(([0, 0, 0, 0], port)));
}
