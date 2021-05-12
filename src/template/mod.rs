use crate::msg::Message;
use std::convert::TryFrom;

pub enum Templates {
    Inoreader,
}

#[derive(Debug)]
pub struct InvalidPayload {}

impl warp::reject::Reject for InvalidPayload {}

impl Templates {
    pub fn render(&self, input: &bytes::Bytes) -> Result<Message, serde_json::Error> {
        match self {
            Self::Inoreader => inoreader::render(input),
        }
    }
}

impl TryFrom<String> for Templates {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match &value[..] {
            "inoreader" => Ok(Self::Inoreader),
            _ => Err("No template found"),
        }
    }
}

mod inoreader;
