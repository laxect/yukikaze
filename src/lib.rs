#![feature(once_cell)]
#[cfg(feature = "server")]
use std::lazy;

#[cfg(feature = "server")]
mod bot;
mod msg;
#[cfg(feature = "server")]
pub mod template;

#[cfg(feature = "server")]
static TOKEN: lazy::SyncLazy<String> = lazy::SyncLazy::new(|| std::env::var("TOKEN").unwrap());
#[cfg(feature = "server")]
static CHAT_ID: lazy::SyncLazy<i64> = lazy::SyncLazy::new(|| std::env::var("CHAT_ID").unwrap().parse().unwrap());

#[cfg(feature = "server")]
pub use bot::send_message;
#[cfg(feature = "message")]
pub use msg::Message;
#[cfg(feature = "server")]
pub use msg::RenderMode;
