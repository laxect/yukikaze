#[cfg(feature = "server")]
pub mod server {
    mod bot;
    pub mod template;
    pub use bot::send_message;
}

mod msg;
pub use msg::{Message, RenderMode};
