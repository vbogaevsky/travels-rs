use std::fmt;
use std::error::Error as StdError;

pub enum Error {
    NotFound,
    InternalServerError
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotFound            => f.write_str("NotFound"),
            Error::InternalServerError => f.write_str("InternalServerError")
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NotFound            => "Record not found",
            Error::InternalServerError => "Internal server error"
        }
    }
}
