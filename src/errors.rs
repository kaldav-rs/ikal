pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("Invalid alarm action: {0}")]
    Alarm(String),
    #[error("{0}")]
    Date(#[from] chrono::ParseError),
    #[error("Invalid freq {0}")]
    Freq(String),
    #[error("Unknow key {0}")]
    Key(String),
    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("{0}")]
    Parser(String),
    #[error("Invalid priority: {0}")]
    Priority(u8),
    #[error("Unknow status {0}")]
    Status(String),
    #[error("Unknow time transparency {0}")]
    TimeTransparency(String),
    #[error("Invalid weekday {0}")]
    Weekday(String),
}

impl<I: std::fmt::Debug> From<nom::Err<nom::error::Error<I>>> for Error {
    fn from(value: nom::Err<nom::error::Error<I>>) -> Self {
        Self::Parser(format!("{value}"))
    }
}
