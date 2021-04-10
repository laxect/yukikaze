use crate::{Message, TOKEN};

const TG_ENDPOINT: &str = "https://api.telegram.org";
fn action(action: &str) -> String {
    format!("{}/bot{}/{}", TG_ENDPOINT, *TOKEN, action)
}

pub async fn send_message(msg: Message) {
    let uri = action("sendMessage");
    let pkg = msg.package();
    let client = reqwest::Client::new();
    let resp = client.post(&uri).json(&pkg).send().await;
    if let Err(e) = resp {
        log::error!("send_message Error: {}", e);
    } else {
        log::info!("send success. {}", resp.unwrap().text().await.unwrap_or_default())
    }
    log::info!("send message");
}
