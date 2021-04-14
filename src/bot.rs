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
        log::error!("{}", e);
    } else {
        let text = resp.unwrap().text().await.unwrap_or_default();
        if text.contains(r#""ok":false"#) {
            log::error!("{}", text);
        } else {
            log::info!("{}", text);
        }
    }
    log::info!("send message");
}
