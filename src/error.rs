#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("P1Error: {0}")]
    P1Error(ftd::p1::Error),
    #[error("IntError: {0}")]
    IntError(std::num::ParseIntError),
    #[error("ValidationError: {0}")]
    ValidationError(String),
    #[error("ColorParseError: {0}")]
    ColorParseError(css_color_parser::ColorParseError),
}

pub fn err<T>(msg: &str) -> Result<T, ParseError> {
    Err(ParseError::ValidationError(msg.to_string()))
}

impl From<css_color_parser::ColorParseError> for ParseError {
    fn from(p: css_color_parser::ColorParseError) -> ParseError {
        ParseError::ColorParseError(p)
    }
}

impl From<std::num::ParseIntError> for ParseError {
    fn from(p: std::num::ParseIntError) -> ParseError {
        ParseError::IntError(p)
    }
}

impl From<ftd::p1::Error> for ParseError {
    fn from(p: ftd::p1::Error) -> ParseError {
        ParseError::P1Error(p)
    }
}

impl From<&str> for ParseError {
    fn from(s: &str) -> ParseError {
        ParseError::ValidationError(s.to_string())
    }
}

impl From<String> for ParseError {
    fn from(s: String) -> ParseError {
        ParseError::ValidationError(s)
    }
}