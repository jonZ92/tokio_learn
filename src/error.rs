/********************************************************************************
 *
 *
 *
 *     ERROR  rust
 *
 *
 *
 *
 *
 *
 *
 ********************************************************************************/

use std::fmt::{Debug, Error, Formatter};
use std::result::Result;
#[derive(Debug)]
pub enum ErrorCode {
    ServerListener,
    ServerAccept,
    BuffOverflow,
    None,
}

pub struct NError {
    pub err_code: ErrorCode,
}

impl NError {
    pub fn new(err_code: ErrorCode) -> Self {
        Self { err_code: err_code }
    }

    pub fn error_description(&self) -> &'static str {
        match self.err_code {
            ErrorCode::ServerListener => return " Server Lister error",
            ErrorCode::ServerAccept => return " Server Accept error",
            ErrorCode::BuffOverflow => return " Server Overflow error",
            _ => return "unkown error",
        }
    }
}

impl std::error::Error for NError {}

impl Debug for NError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "NError[{:?}]", self.err_code)
    }
}

/*******************************
 *
 *
 * 自定义  打印 格式
 *
 * println!("{}",NError::new(1));
 *
 * print out the error message
 *
 * NError[code , mag ]
 *
 ********************************/
impl std::fmt::Display for NError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "NError[ {} ]", self.error_description())
    }
}

#[cfg(test)]
mod tests_ {
    use super::*;
    #[test]
    fn test_() {
        let er = NError::new(ErrorCode::ServerListener);
        println!("{} ", er);
    }
}
