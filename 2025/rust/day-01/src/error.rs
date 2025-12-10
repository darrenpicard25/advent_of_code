#[derive(Debug)]
pub enum AocError {
    IoError,
}

pub type AocResult<T> = Result<T, AocError>;
