use std::fs::{File, OpenOptions};
use std::path::Path;

pub struct DataLog {
    pub path: Path,                        // path to the data log file
    pub file: File,                        // file handle
    pub(crate) currrent_offset: u64,       // current offset in the file
    pub(crate) free_slots: Vec<FreeSlots>, // list of free slots in the file
}

pub struct FreeSlots {
    pub offset: u64, // offset of the free slot
    pub length: u32, // length of the free slot
}

impl DataLog {
    pub fn new(path: Path) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        Ok(DataLog {
            path,
            file,
            currrent_offset: 0,
            free_slots: Vec::new(),
        })
    }
}
