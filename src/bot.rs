use crate::{Message, TOKEN};

const TG_ENDPOINT: &str = "https://api.telegram.org";
fn action(action: &str) -> String {
    format!("{}/bot{}/{}", TG_ENDPOINT, *TOKEN, action)
}

pub async fn send_message(msg: Message) {
    let uri = action("sendMessage");
    let body = msg.package();
    let client = reqwest::Client::new();
    if let Err(e) = client.post(&uri).json(&body).send().await {
        log::error!("send_message Error: {}", e);
    }
    log::info!("send message");
}
