use std::fmt;

pub struct Error {
    message: String
}

impl Error {
    pub fn new<S: Into<String>>(message: S) -> Error {
        Error {
            message: message.into(),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}