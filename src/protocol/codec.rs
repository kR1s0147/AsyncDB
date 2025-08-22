use super::*;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::Framed;
use tokio_util::codec::{Decoder, Encoder};

pub struct Codec;

impl Decoder for Codec {
    type Item = Command;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match Command::read(src) {
            Ok(command) => Some(command),
            Error::InsufficientBytes(remaining_bytes) => {
                src.reserve(remaining_bytes as usize);
                return Ok(None);
            }
            Err(e) => Err(e),
        }
    }
}

impl Encoder<Command> for Codec {
    type Error = Error;
    fn encode(&mut self, item: Command, src: &mut BytesMut) -> Result<(), Self::Error> {
        item.write(src)?;
        Ok(())
    }
}

pub struct NetWorkCodec {
    framed: Framed<Box<dyn AsyncRead + AsyncWrite + Send + Unpin>, Codec>,
}

impl NetWorkCodec {
    pub fn new(stream: Box<dyn AsyncRead + AsyncWrite + Send + Unpin>) -> Self {
        let framed = Framed::new(stream, Codec);
        NetWorkCodec { framed }
    }

    pub async fn send(&mut self, command: Command) -> Result<(), Error> {
        self.framed.send(command).await
    }

    pub async fn receive(&mut self) -> Result<Option<Command>, Error> {
        self.framed.next().await.transpose()
    }
}
