use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct RpcError(pub String);

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for RpcError {}
