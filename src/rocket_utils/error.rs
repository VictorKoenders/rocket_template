use rocket::http::Status;
use std::fmt;

pub struct Error(failure::Error);

impl<T> From<T> for Error
where
    T: Into<failure::Error>,
{
    fn from(item: T) -> Error {
        Error(item.into())
    }
}

impl Error {
    pub fn from_status_code(status: Status) -> Error {
        Error(failure::format_err!("{:?}", status))
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
pub type ResponseResult = Result<rocket::Response<'static>>;
