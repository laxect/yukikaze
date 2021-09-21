use serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};

#[cfg(feature = "message")]
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum RenderMode {
    Markdown,
    Html,
    None,
}

#[cfg(feature = "message")]
impl Default for RenderMode {
    fn default() -> Self {
        Self::None
    }
}

#[cfg(feature = "message")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub msg: String,
    pub node: String,
    #[serde(default)]
    pub mode: RenderMode,
    #[serde(default)]
    pub disable_notification: bool, // default to false
}

#[cfg(feature = "server")]
mod message_impl {
    use super::{Message, RenderMode};
    use once_cell::sync::Lazy;
    use pulldown_cmark::{html, Options, Parser};
    use telegram_types::bot::{
        methods::{ChatTarget, SendMessage},
        types::{ChatId, ParseMode},
    };

    static CHAT_ID: Lazy<i64> = Lazy::new(|| std::env::var("CHAT_ID").unwrap().parse().unwrap());

    impl From<RenderMode> for Option<ParseMode> {
        fn from(mode: RenderMode) -> Option<ParseMode> {
            match mode {
                RenderMode::Markdown | RenderMode::Html => ParseMode::HTML.into(),
                RenderMode::None => None,
            }
        }
    }

    impl Message {
        pub(crate) fn render(&self) -> String {
            let mut msg = if self.mode == RenderMode::Markdown {
                let mut options = Options::empty();
                options.insert(Options::ENABLE_STRIKETHROUGH);
                let parser = Parser::new_ext(&self.msg, options);
                let mut buffer = String::new();
                html::push_html(&mut buffer, parser);
                buffer
            } else {
                self.msg.clone()
            };
            msg.pop();
            let msg = msg.replace("<p>", "").replace("</p>", "\n");
            format!("{}\n---\n{} - 雪風改", msg, self.node)
        }

        pub(crate) fn parse_mode(&self) -> Option<ParseMode> {
            self.mode.into()
        }

        pub(crate) fn package(self) -> SendMessage<'static> {
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
}
