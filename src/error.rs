use derive_more::From;
pub type Result<T> = core::result::Result<T, Error>;

/// Main GenAI error
#[derive(Debug, From)]
#[allow(missing_docs)]
pub enum Error {    
    #[from]
    NetworkError {
        message: String
    }        
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Error::NetworkError { message } => write!(f, "Network error: {}", message),            
        }
    }
}

impl std::error::Error for Error {}