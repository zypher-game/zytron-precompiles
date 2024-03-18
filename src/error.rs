#[derive(Debug)]
pub enum Error {
    Serialize,
    Deserialize,
}

impl Error {
    pub fn code(&self) -> u8 {
        match self {
            Error::Serialize => 1,
            Error::Deserialize => 2,
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
