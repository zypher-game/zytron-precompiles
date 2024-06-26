#[derive(Debug)]
pub enum Error {
    Serialize,
    Deserialize,
    VerifyFail,
    Unknown,
}

impl Error {
    pub fn code(&self) -> u8 {
        match self {
            Self::Serialize => 1,
            Self::Deserialize => 2,
            Self::VerifyFail => 3,
            Self::Unknown => 4,
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
