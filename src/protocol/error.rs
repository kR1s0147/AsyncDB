use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Insufficent bytes to read")]
    InsufficientBytes(u64),
}
