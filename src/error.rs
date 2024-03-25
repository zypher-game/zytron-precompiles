#[derive(Debug)]
pub enum Error {
    Serialize,
    Deserialize,
    VerifyFail,
}

impl Error {
    pub fn code(&self) -> u8 {
        match self {
            Self::Serialize => 1,
            Self::Deserialize => 2,
            Self::VerifyFail => 3,
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
