use bytes::{Bytes, BytesMut};

mod codec;
mod delete;
mod get;
mod header;
mod set;
mod update;

pub enum Command {
    SET(set::SET),
    GET(get::GET),
    DELETE(delete::DELETE),
    UPDATE(update::UPDATE),
}

impl Command {
    pub fn read(bytes: Bytes) -> Result<Command, Error> {
        let mut iter = bytes.iter();
        let header = header::Header::parse_header(&mut iter)?;

        match header.command {
            0 => Ok(Command::SET(set::SET::read(header, bytes)?)),
            1 => Ok(Command::GET(get::GET::read(header, bytes)?)),
            2 => Ok(Command::DELETE(delete::DELETE::read(header, bytes)?)),
            3 => Ok(Command::UPDATE(update::UPDATE::read(header, bytes)?)),
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Unknown command",
            ))),
        }
    }
}
