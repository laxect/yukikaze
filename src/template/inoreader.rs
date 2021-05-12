use crate::{msg::RenderMode, Message};
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct InoreaderWebhook {
    items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Item {
    summary: Summary,
}

#[derive(Debug, Deserialize)]
struct Summary {
    content: String,
}

pub(super) fn render(input: &bytes::Bytes) -> Result<Message, serde_json::Error> {
    let input: InoreaderWebhook = serde_json::from_slice(input)?;
    let content = input.items.into_iter().next().unwrap().summary.content; // should always have at least one item
    log::info!("{}", content);
    let content = Regex::new(r#" *Ino.*?detected *"#).unwrap().replace_all(&content, "");
    let content = Regex::new(r#"(<\\p>)|(<br>)"#).unwrap().replace_all(&content, "\n");
    let content = Regex::new(r#"(<.*?>)"#).unwrap().replace_all(&content, "");
    let content = Regex::new(r#"( *\n *\n *)+"#).unwrap().replace_all(&content, "\n");
    let content = Regex::new(r#"(^\n)|(\n$)"#).unwrap().replace_all(&content, "");
    log::info!("{}", content);
    let msg = Message {
        msg: content.to_string(),
        node: "深夜".to_string(),
        disable_notification: false,
        mode: RenderMode::Markdown,
    };
    Ok(msg)
}
