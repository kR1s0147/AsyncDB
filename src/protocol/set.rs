use std::fmt::Error;

use super::*;
use bytes::Buf;

pub struct SET {
    pub key_length: u16,
    pub key: String,
    pub value_lenght: u16,
    pub value: String,
    pub ttl: Option<u64>,
}

impl SET {
    pub fn new(
        key_length: u16,
        key: String,
        value_length: u16,
        value: String,
        ttl: Option<u32>,
    ) -> Self {
        SET {
            key_length,
            key,
            value_lenght: value_length,
            value,
            ttl,
        }
    }

    pub fn read(header: Header, mut bytes: Bytes) -> Result<SET, Error> {
        let header = Header::parse_header(bytes.iter())?;
        bytes.advance(3);
        if bytes.len() < (header.length as usize - 3) {
            return Err(Error::new("Not enough data to read SET command"));
        }

        let key_length = bytes.get_u16() as u16;
        let key = bytes.split_to(key_length as usize);
        let value_length = bytes.get_u16() as u16;
        let value = bytes.split_to(value_length as usize);
        let ttl = if bytes.has_remaining() {
            Some(bytes.get_u32())
        } else {
            None
        };

        let key = String::from_utf8(key.to_vec())
            .map_err(|_| Error::new("Invalid UTF-8 sequence in key"))?;
        let value = String::from_utf8(value.to_vec())
            .map_err(|_| Error::new("Invalid UTF-8 sequence in value"))?;
    }

    pub fn size(&self) -> usize {
        3 + // Header size
        2 + // Key length
        self.key_length as usize + // Key length
        2 + // Value length
        self.value_lenght as usize + // Value length
        if let Some(_) = self.ttl { 4 } else { 0 } // TTL size, if present
    }

    pub fn write(&self, src: &mut BytesMut) -> Result<(), Error> {
        let size = self.size();
        let mut bytes = src;
        bytes.put_u8(0); // Command type for SET
        bytes.put_u16(size as u16 - 3); // Length of the rest of
        bytes.put_u16(self.key_length);
        bytes.put_slice(self.key.as_bytes());
        bytes.put_u16(self.value_lenght);
        bytes.put_slice(self.value.as_bytes());
        if let Some(ttl) = self.ttl {
            bytes.put_u32(ttl);
        }
        Ok(())
    }
}
