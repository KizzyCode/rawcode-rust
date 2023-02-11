//! Implements the crate's error type

use core::fmt::{self, Display, Formatter};

/// Creates a new variant
#[macro_export]
#[doc(hidden)]
macro_rules! e {
    ($message:expr) => {{
        $crate::error::Error::new($message, file!(), line!())
    }};
}

/// A error type
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Error {
    /// The error message
    message: &'static str,
    /// The file where the error originated
    file: &'static str,
    /// The line where the error originated
    line: u32,
}
impl Error {
    /// Creates a new error
    #[doc(hidden)]
    pub const fn new(message: &'static str, file: &'static str, line: u32) -> Self {
        Self { message, file, line }
    }

    /// The error message
    pub const fn message(&self) -> &'static str {
        self.message
    }

    /// The error location
    pub const fn location(&self) -> (&'static str, u32) {
        (self.file, self.line)
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} at {}:{}", self.message, self.file, self.line)
    }
}
#[cfg(feature = "std")]
impl std::error::Error for Error {
    /* No members to implement */
}
