#[cfg(feature = "server")]
mod bot;
mod msg;
#[cfg(feature = "server")]
pub mod template;

#[cfg(feature = "server")]
pub use bot::send_message;
#[cfg(feature = "message")]
pub use msg::{Message, RenderMode};
