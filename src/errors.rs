pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("Unknow class {0}")]
    Class(String),
    #[error("{0}")]
    Date(#[from] chrono::ParseError),
    #[error("Unknow key {0}")]
    Key(String),
    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("{0}")]
    Parser(String),
    #[error("Unknow status {0}")]
    Status(String),
}
