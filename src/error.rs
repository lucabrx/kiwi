#[derive(Debug)]
pub enum DBError {
    NotFound,
    Expired,
    Internal,
    ParseError,
    InvalidArg,
}

impl std::fmt::Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DBError::NotFound => write!(f, "Key not found"),
            DBError::Expired => write!(f, "Key has expired"),
            DBError::Internal => write!(f, "Internal error"),
            DBError::ParseError => write!(f, "Parse error"),
            DBError::InvalidArg => write!(f, "Invalid argument"),
        }
    }
}
