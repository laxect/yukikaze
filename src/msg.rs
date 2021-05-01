use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use std::cmp::{Eq, PartialEq};
use telegram_types::bot::{
    methods::{ChatTarget, SendMessage},
    types::{ChatId, ParseMode},
};

use crate::CHAT_ID;

#[derive(Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
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
            RenderMode::Markdown | RenderMode::Html => ParseMode::HTML.into(),
            RenderMode::None => None,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    msg: String,
    node: String,
    #[serde(default)]
    mode: RenderMode,
    #[serde(default)]
    disable_notification: bool, // default to false
}

impl Message {
    pub fn render(&self) -> String {
        let mut msg = if self.mode == RenderMode::Markdown {
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            let parser = Parser::new_ext(&self.msg, options);
            let mut html_out = String::new();
            html::push_html(&mut html_out, parser);
            html_out
        } else {
            self.msg.clone()
        };
        msg.pop();
        let msg = msg.replace("<p>", "").replace("</p>", "\n");
        format!("{}\n---\n{} - 雪風改", msg, self.node)
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
