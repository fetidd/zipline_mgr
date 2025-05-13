use std::io::Read;

pub const BIND_ADDR_WINDS: &str = "127.0.0.1:8888";
pub const BIND_ADDR_LINE: &str = "127.0.0.1:8889";

pub struct Setup {
    black: bool,
    old_red_front: bool,
    old_red_middle: bool,
    new_red: bool,
    yellow: bool,
    weights: u8,
}

pub enum Message {
    Weights(u8, u8, u8, u8),
    ConfirmationRequest((Setup, u8), (Setup, u8), (Setup, u8), (Setup, u8)),
}

impl Message {
    pub fn new_weights(data: Vec<u8>) -> Self {
        Self::Weights(0, 1, 2, 3)
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Message::Weights(_, _, _, _) => "weights",
                Message::ConfirmationRequest(_, _, _, _) => "confirm?",
            }
        )
    }
}

pub fn parse_message_type(raw: &Vec<u8>) -> Result<&fn(Vec<u8>) -> Message, CommunicationError> {
    let raw_msg_type = String::from_utf8_lossy(&raw).into_owned();
    println!("{raw_msg_type}");
    match raw_msg_type.as_str() {
        "0100" => Ok(&(Message::new_weights as fn(Vec<u8>) -> Message)),
        _ => Err(CommunicationError::TODO),
    }
}

#[derive(Debug)]
pub enum CommunicationError {
    TODO,
}
