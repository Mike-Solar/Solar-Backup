use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

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