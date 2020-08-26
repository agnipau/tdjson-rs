use {
    std::{ffi, str},
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum TdlibError {
    #[error("tdlib sent an invalid utf-8 string")]
    Utf8Error(str::Utf8Error),

    #[error("Null characters in request string")]
    NulError(ffi::NulError),

    #[cfg(feature = "types")]
    #[error("Request couldn't be serialized by serde")]
    InvalidRequestData(serde_json::Error),
}
