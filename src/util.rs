use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct AocError;

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oooooops!")
    }
}

impl error::Error for AocError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
