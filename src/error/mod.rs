use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ErrorHandler {
    details: String
}

impl ErrorHandler {
    fn new(message: &str) -> ErrorHandler {
        ErrorHandler{details: message.to_string()}
    }
}

impl fmt::Display for ErrorHandler {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter,"{}",self.details)
    }
}

impl Error for ErrorHandler {
    fn description(&self) -> &str {
        &self.details
    }
}

// TODO: write tests
#[cfg(test)]
mod tests {
    use super::*;


}