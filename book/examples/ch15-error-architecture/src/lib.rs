use std::error::Error;
use std::fmt;
use std::num::ParseFloatError;

pub type Result<T> = std::result::Result<T, ParsePriceError>;

#[derive(Debug)]
pub enum ParsePriceError {
    EmptyField,
    InvalidFloat(ParseFloatError),
    NonPositive(f64),
}

impl fmt::Display for ParsePriceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsePriceError::EmptyField => write!(formatter, "price field is empty"),
            ParsePriceError::InvalidFloat(error) => write!(formatter, "invalid float: {error}"),
            ParsePriceError::NonPositive(value) => {
                write!(formatter, "price is not positive: {value}")
            }
        }
    }
}

impl Error for ParsePriceError {}

impl From<ParseFloatError> for ParsePriceError {
    fn from(error: ParseFloatError) -> Self {
        Self::InvalidFloat(error)
    }
}

pub fn parse_price(text: &str) -> Result<f64> {
    if text.trim().is_empty() {
        return Err(ParsePriceError::EmptyField);
    }

    let value = text.parse::<f64>()?;

    if value <= 0.0 {
        return Err(ParsePriceError::NonPositive(value));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_conversion_keeps_question_mark_useful() {
        assert_eq!(parse_price("100.5").expect("valid price"), 100.5);
        assert!(matches!(parse_price(""), Err(ParsePriceError::EmptyField)));
        assert!(matches!(
            parse_price("-1"),
            Err(ParsePriceError::NonPositive(-1.0))
        ));
    }
}
