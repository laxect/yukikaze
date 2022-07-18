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
    let content = html2md::parse_html(&content);
    log::info!("{}", content);
    let msg = Message {
        msg: content,
        node: "深夜".to_string(),
        mode: RenderMode::Markdown,
        disable_notification: false,
    };
    Ok(msg)
}
