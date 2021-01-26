use serde::Deserialize;
use telegram_types::bot::{
    methods::{ChatTarget, SendMessage},
    types::{ChatId, ParseMode},
};

use crate::CHAT_ID;

#[derive(Deserialize, Copy, Clone)]
pub enum RenderMode {
    Markdown,
    Html,
    None,
}

impl Default for RenderMode {
    fn default() -> Self {
        Self::None
    }
}

impl From<RenderMode> for Option<ParseMode> {
    fn from(mode: RenderMode) -> Option<ParseMode> {
        match mode {
            RenderMode::Markdown => ParseMode::Markdown.into(),
            RenderMode::Html => ParseMode::HTML.into(),
            RenderMode::None => None,
        }
    }
}

#[derive(Deserialize)]
pub struct Message {
    msg: String,
    node: String,
    #[serde(default)]
    mode: RenderMode,
    #[serde(default)]
    disable_notification: bool,
}

impl Message {
    pub fn render(&self) -> String {
        format!("{}\n\n---\n{} - 雪風改", self.msg, self.node)
    }

    pub fn parse_mode(&self) -> Option<ParseMode> {
        self.mode.into()
    }

    pub fn package(self) -> SendMessage<'static> {
        let chat_id = ChatTarget::Id(ChatId(*CHAT_ID));
        let mut msg = SendMessage::new(chat_id, self.render());
        if let Some(parse_mode) = self.parse_mode() {
            msg = msg.parse_mode(parse_mode);
        }
        if self.disable_notification {
            msg.disable_notification = Some(true);
        }
        msg
    }
}
