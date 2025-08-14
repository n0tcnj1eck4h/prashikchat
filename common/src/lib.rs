use bincode::{
    Decode, Encode,
    error::{DecodeError, EncodeError},
};

#[derive(Encode, Decode, Debug)]
pub struct CreateGuild {
    name: String,
}

#[derive(Encode, Decode, Debug)]
pub enum ClientMessage {
    CreateGuild(CreateGuild),
}

impl ClientMessage {
    pub fn encode(self) -> Result<Vec<u8>, EncodeError> {
        bincode::encode_to_vec(self, bincode::config::standard())
    }

    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(bincode::decode_from_slice(data, bincode::config::standard())?.0)
    }
}

#[derive(Encode, Decode, Debug)]
pub struct JoinGuild {
    id: u32,
    name: String,
    icon_url: String,
}

#[derive(Encode, Decode, Debug)]
pub enum ServerMessage {
    JoinGuild(JoinGuild),
}

impl ServerMessage {
    pub fn encode(self) -> Result<Vec<u8>, EncodeError> {
        bincode::encode_to_vec(self, bincode::config::standard())
    }

    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(bincode::decode_from_slice(data, bincode::config::standard())?.0)
    }
}
