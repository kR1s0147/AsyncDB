use std::slice::Iter;

pub struct Header {
    // Command Type (SET, GET, etc.)
    pub command: u8,

    // lenght of the rest of the header
    pub length: u16,
}

impl Header {
    pub fn new(command: u8, length: u16) -> Self {
        Header { command, length }
    }

    pub fn size(&self) -> usize {
        1  // 1 byte for command
        +
        2 // 2 bytes for length 
    }

    pub fn total_size(&self) -> usize {
        self.size() + self.length as usize
    }

    /// Reads a header from the provided byte slice.
    pub fn parse_header(mut src: Iter<u8>) -> Result<Header, Error> {
        if src.len() < 3 {
            return Err(Error::new("Not enough data to read header"));
        }
        let command = src.next().unwrap();
        let mut len: u16 = 0;
        let mut shift = 0;

        for i in 0..2 {
            let byte = *src.next().unwrap() as u16;
            len += byte << shift; // Read the next two bytes as length
            shift += 8;
        }

        if src.len() < (len as usize + 3) {
            return Err(Error::new("Not enough data to read the full header"));
        }

        Ok(Self::new(*command, len))
    }
}
