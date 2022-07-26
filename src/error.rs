use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

use serenity::Error as SerenityError;

/// A common error type for all functions and methods of the library.
///
/// It can be directly converted into serenity's [`Error`](SerenityError).
#[derive(Debug)]
pub enum Error {
    /// Error returned by serenity.
    SerenityError(SerenityError),
    /// Error returned when an operation times out.
    TimeoutError,
    /// Error returned when user's choice is invalid.
    InvalidChoice,
    /// Error returned for all other cases.
    Other(String),
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let err = match self {
            Error::SerenityError(e) => Cow::from(e.to_string()),
            Error::TimeoutError => Cow::from("You took too long to respond."),
            Error::InvalidChoice => Cow::from("Invalid choice!"),
            Error::Other(e) => Cow::from(e),
        };

        write!(f, "{}", err)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(error: &'a str) -> Self {
        Self::Other(error.to_string())
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Self::Other(error)
    }
}

impl From<SerenityError> for Error {
    fn from(error: SerenityError) -> Self {
        Self::SerenityError(error)
    }
}
