#![feature(once_cell)]
use std::lazy;

mod bot;
mod msg;
pub mod template;

static TOKEN: lazy::SyncLazy<String> = lazy::SyncLazy::new(|| std::env::var("TOKEN").unwrap());
static CHAT_ID: lazy::SyncLazy<i64> = lazy::SyncLazy::new(|| std::env::var("CHAT_ID").unwrap().parse().unwrap());

pub use bot::send_message;
pub use msg::{Message, RenderMode};
