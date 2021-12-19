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
    EmptyData,
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
            ErrorCode::EmptyData => return "Empty data Exception",
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
        let er = NError::new(ErrorCode::None);
        println!("{} ", er);
    }
}
