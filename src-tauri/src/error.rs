use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};

pub enum ErrorType{
    LoadConfigError =1,
    UnInitedBackup=2,
}
#[derive(Serialize, Deserialize)]
pub struct ErrorResult{
    pub error: String,
    pub error_type: u8,
}

impl ErrorResult{
    pub fn new(error: &dyn Error, error_type: ErrorType) -> Self{
        return ErrorResult{
            error: error.to_string(),
            error_type:error_type as u8,
        }
    }
}

pub struct NotInitedBackupError{

}

impl NotInitedBackupError{
    pub(crate) fn new() -> Self {return NotInitedBackupError{};}
}

impl Debug for NotInitedBackupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NotInitedBackupError")
    }
}

impl Display for NotInitedBackupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NotInitedBackupError")
    }
}

impl Error for NotInitedBackupError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        return None;
    }
}