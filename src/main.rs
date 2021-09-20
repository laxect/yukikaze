fn main() {
    #[cfg(feature = "server")]
    {
        use stackdriver_logger::Service;
        use std::convert::{Infallible, TryFrom};
        use warp::Filter;
        use yukikaze::{send_message, template, template::InvalidPayload, Message};

        async fn handle(msg: Message) -> Result<&'static str, Infallible> {
            send_message(msg).await;
            log::info!("handle done");
            Ok("done")
        }

        async fn handle_template(template: String, msg: bytes::Bytes) -> Result<&'static str, warp::reject::Rejection> {
            let template = template::Templates::try_from(template).map_err(|e| {
                log::error!("{}", e);
                warp::reject::not_found()
            })?;
            let msg = template.render(&msg).map_err(|e| {
                log::error!("{}", e);
                warp::reject::custom(InvalidPayload {})
            })?;
            send_message(msg).await;
            log::info!("handle done");
            Ok("done")
        }

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
        let root = warp::path::end()
            .and(warp::post())
            .and(warp::body::json())
            .and_then(handle);
        let template = warp::path!("t" / String)
            .and(warp::post())
            .and(warp::body::bytes())
            .and_then(handle_template)
            .with(warp::log::log("yukikaze::tepmlate"));
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(warp::serve(root.or(template)).run(([0, 0, 0, 0], port)));
    }
}
