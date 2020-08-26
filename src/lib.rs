mod client;
mod error;

pub use client::iter::ClientIter;
#[cfg(feature = "types")]
pub use client::iter::TypedClientIter;

pub use client::receive::ReceiveClient;
pub use client::safe::Client;
pub use client::send::SendClient;

use {
    crate::error::TdlibError,
    std::{ffi::CString, str},
};

/// Sets the path to the file where the internal TDLib log will be written.
///
/// By default TDLib writes logs to stderr or an OS specific log. Use this method to write the log to a file instead.
///
/// # Parameters
///
/// `path` Maybe path to a file where the internal TDLib log will be written. Use `None` to switch back to the default logging behaviour.
///
/// # Examples
///
/// ```
/// set_log_file_path(Some("/var/log/tdlib/tdlib.log"));
/// ```
pub fn set_log_file_path(path: &str) -> Result<i32, TdlibError> {
    let cpath = CString::new(path).map_err(TdlibError::NulError)?;
    Ok(unsafe { tdjson_sys::td_set_log_file_path(cpath.as_ptr()) })
}

/// Sets the verbosity level of the internal logging of TDLib.
///
/// By default the TDLib uses a log verbosity level of 5.
///
/// # Parameters
///
/// `level` New value of logging verbosity level. Value 0 corresponds to fatal errors,
/// value 1 corresponds to errors, value 2 corresponds to warnings and debug warnings,
/// value 3 corresponds to informational, value 4 corresponds to debug, value 5 corresponds
/// to verbose debug, value greater than 5 and up to 1024 can be used to enable even more logging.
///
/// # Examples
///
/// ```
/// set_log_verbosity_level(3);
/// ```
pub fn set_log_verbosity_level(level: i32) {
    unsafe {
        tdjson_sys::td_set_log_verbosity_level(level);
    }
}

/// Sets maximum size of the file to where the internal TDLib log is written before the file will be auto-rotated.
///
/// Unused if log is not written to a file. Defaults to 10 MB.
///
/// # Parameters
///
/// `size` Maximum size of the file to where the internal TDLib log is written before the file will be auto-rotated. Should be positive.
///
/// # Examples
///
/// ```
/// set_log_max_file_size(1024 * 1024);
/// ```
pub fn set_log_max_file_size(size: i64) {
    unsafe { tdjson_sys::td_set_log_max_file_size(size) }
}
