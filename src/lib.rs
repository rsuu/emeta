pub mod lib {
    pub mod meta;
    pub mod tags;
}

pub use crate::lib::*;

pub type SelfResult<T> = Result<T, MyError>;
pub type MyResult = SelfResult<()>;

// tags
pub type Names = Vec<String>;

#[derive(Debug)]
pub enum MyError {
    ErrCanvas(Box<dyn std::error::Error>),   // canvas
    ErrEvent(String),                        // event
    ErrIo(std::io::Error),                   // io
    ErrFromUtf8(std::string::FromUtf8Error), // utf8
    ErrMiniserde(miniserde::Error),          //
    ErrSpeedy(speedy::Error),                // metadata
    ErrTryFromSlice(std::array::TryFromSliceError),
    ErrNull(()),
    ErrMagickNumber,
}

impl From<String> for MyError {
    fn from(e: String) -> Self {
        MyError::ErrEvent(e)
    }
}

impl From<()> for MyError {
    fn from(e: ()) -> Self {
        MyError::ErrNull(e)
    }
}

impl From<std::array::TryFromSliceError> for MyError {
    fn from(e: std::array::TryFromSliceError) -> Self {
        MyError::ErrTryFromSlice(e)
    }
}

impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        MyError::ErrIo(e)
    }
}

impl From<speedy::Error> for MyError {
    fn from(e: speedy::Error) -> Self {
        MyError::ErrSpeedy(e)
    }
}

impl From<miniserde::Error> for MyError {
    fn from(e: miniserde::Error) -> Self {
        MyError::ErrMiniserde(e)
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for MyError {
    fn from(e: std::boxed::Box<dyn std::error::Error>) -> Self {
        MyError::ErrCanvas(e)
    }
}
