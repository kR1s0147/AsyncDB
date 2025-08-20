use super::*;
use tokio_util::codec::Framed;
use tokio_util::codec::{Decoder, Encoder};

pub struct Codec;

impl Decoder for Codec {
    type Item = Command;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 3 {
            return Ok(None); // Not enough data to read header
        }

        let command = Command::read(src.clone().freeze());
        src.advance(command.total_size());
        Ok(Some(command))
    }
}
