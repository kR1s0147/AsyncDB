use super::*;
use bytes::Buf;

pub struct GET {
    pub key_length: u16,
    pub key: String,
}

impl GET {
    pub fn new(key_length: u16, key: String) -> Self {
        GET { key_length, key }
    }

    pub fn read(header: Header, mut bytes: Bytes) -> Result<GET, Error> {
        let header = Header::parse_header(bytes.iter())?;
        bytes.advance(3);
        if bytes.len() < (header.length as usize - 3) {
            return Err(Error::new("Not enough data to read GET command"));
        }

        let key_length = bytes.get_u16() as u16;
        let key = bytes.split_to(key_length as usize);

        let key = String::from_utf8(key.to_vec())
            .map_err(|_| Error::new("Invalid UTF-8 sequence in key"))?;

        Ok(GET::new(key_length, key))
    }

    pub fn size(&self) -> usize {
        3 + // Header size
        2 + // Key length
        self.key_length as usize // Key length
    }

    pub fn write(&self, src: &mut BytesMut) -> Result<(), Error> {
        let size = self.size();
        let mut bytes = src;
        bytes.put_u8(1); // Command type for GET
        bytes.put_u16(size as u16 - 3); // Length of the rest of
        bytes.put_u16(self.key_length);
        bytes.put_slice(self.key.as_bytes());
        Ok(())
    }
}
