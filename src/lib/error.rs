use std::error::Error as StdError;
use std::convert::From;
use std::fmt;
use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::Request;
use rocket::response::{Response, Responder};


#[derive(Debug)]
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


impl From<DieselError> for Error {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => Error::NotFound,
            _                     => Error::InternalServerError
        }
    }
}

// impl<'r> Responder<'r> for Error {
impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> Result<Response<'r>, Status> {
        match self {
            Error::NotFound => Err(Status::NotFound),
            _               => Err(Status::InternalServerError)
        }
    }
}
