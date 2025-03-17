use std::error::Error;

pub struct NotInitedBackupError{

}
impl Error for NotInitedBackupError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        return None;
    }
}