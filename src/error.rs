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

use std::fmt::{Error, Formatter};
use std::result::Result;
pub const ERROR_PARSM: i32 = 1;
#[derive(Debug)]
pub struct NError {
    pub err_code: i32,
}

impl NError {
    pub fn new(err_code: i32) -> Self {
        Self { err_code }
    }

    pub fn error_description(&self) -> &'static str {
        match self.err_code {
            ERROR_PARSM => return "parse error",
            _ => return "unkown error",
        }
    }
}

impl std::error::Error for NError {}

impl std::fmt::Display for NError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "NError[{},{}]", self.err_code, self.error_description())
    }
}
