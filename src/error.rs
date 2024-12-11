use core::{fmt, num};
use std::io;

/// Errors for `Reader` and `Writer`.
#[derive(Debug)]
pub enum Error {
    /// A downstream IO error.
    IO(io::Error),

    /// A downstream parsing error.
    Parse {
        /// The parser error
        error: num::ParseIntError,
        /// The line number in the font file this was encountered on
        line_number: u32,
        /// The contents of the line that this error was encountered on
        line: String,
    },

    /// `STARTFONT` is missing the format version.
    MissingVersion {
        /// The line number in the font file this was encountered on
        line_number: u32,
        /// The contents of the line that this error was encountered on
        line: String,
    },

    /// There was no bounding box for a character.
    MissingBoundingBox {
        /// The line number in the font file this was encountered on
        line_number: u32,
        /// The contents of the line that this error was encountered on
        line: String,
    },

    /// An entry is missing a value.
    MissingValue {
        /// The name of the property that was missing a value
        property_name: String,
        /// The contents of the line that this error was encountered on
        line_number: u32,
    },

    /// An unknown error.
    InvalidCodepoint {
        /// The line number in the font file this was encountered on
        line_number: u32,
        /// The contents of the line that this error was encountered on
        line: String,
    },

    /// EOF has been reached.
    End,

    /// The font declaration is malformed.
    MalformedFont,

    /// The property declarations are malformed.
    MalformedProperties,

    /// The character declaration is malformed.
    MalformedChar,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IO(err) => write!(f, "{}", err),
            Error::Parse {
                error,
                line_number,
                line,
            } => write!(f, "{} on line {}: `{}`", error, line_number, line),
            Error::MissingVersion { line_number, line } => write!(
                f,
                "Missing version from STARTFONT on line {}: {}",
                line_number, line
            ),
            Error::MissingBoundingBox { line_number, line } => {
                write!(f, "Missing bounding box on line {}: {}", line_number, line)
            }
            Error::MissingValue {
                property_name,
                line_number,
            } => write!(
                f,
                "Missing value for property `{}` on line {}",
                property_name, line_number
            ),
            Error::InvalidCodepoint { line_number, line } => write!(
                f,
                "An invalid codepoint has been found on line {}: {}",
                line_number, line
            ),
            Error::End => write!(f, "End of file reached"),
            Error::MalformedFont => write!(f, "Malformed font definition"),
            Error::MalformedProperties => write!(f, "Malformed properties definition"),
            Error::MalformedChar => write!(f, "Malformed character definition"),
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Error::IO(err) => Some(err),
            Error::Parse { error, .. } => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(io: io::Error) -> Self {
        Error::IO(io)
    }
}
