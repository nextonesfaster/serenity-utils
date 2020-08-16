use serenity::Error as SerenityError;
use std::{
    error::Error as StdError,
    fmt::{self, Display, Formatter},
};

/// A common error type for all functions and methods of the library.
///
/// It can be directly converted into serenity's `Error`.
#[derive(Clone, Debug)]
pub struct Error(pub String);

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(error: &'a str) -> Self {
        Self(error.to_string())
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Self(error)
    }
}

impl From<SerenityError> for Error {
    fn from(error: SerenityError) -> Self {
        Self(error.to_string())
    }
}
