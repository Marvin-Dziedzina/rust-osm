#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The value is out of range")]
    OutOfRange,
}
