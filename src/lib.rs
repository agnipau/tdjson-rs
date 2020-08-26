use {
    std::{
        ffi::{self, CStr, CString},
        os::raw::{c_char, c_void},
        str,
        sync::Arc,
        time::Duration,
    },
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

#[derive(Debug)]
struct UnsafeClient {
    client_ptr: *mut c_void,
}

impl UnsafeClient {
    fn new() -> Self {
        UnsafeClient {
            client_ptr: unsafe { tdjson_sys::td_json_client_create() },
        }
    }

    /// UNSAFE: the returned slice is invalidated upon the next call to execute
    /// or receive.
    unsafe fn execute(&self, request: &str) -> Result<Option<&str>, TdlibError> {
        let crequest = CString::new(request).map_err(TdlibError::NulError)?;
        let answer = tdjson_sys::td_json_client_execute(self.client_ptr, crequest.as_ptr());
        let answer = answer as *const c_char;
        if answer.is_null() {
            return Ok(None);
        }
        let answer = CStr::from_ptr(answer);
        Ok(Some(answer.to_str().map_err(TdlibError::Utf8Error)?))
    }

    fn send(&self, request: &str) -> Result<(), TdlibError> {
        let crequest = CString::new(request).map_err(TdlibError::NulError)?;
        unsafe { tdjson_sys::td_json_client_send(self.client_ptr, crequest.as_ptr()) }
        Ok(())
    }

    /// UNSAFE: the returned slice is invalidated upon the next call to execute
    /// or receive.
    unsafe fn receive(&self, timeout: Duration) -> Result<Option<&str>, TdlibError> {
        let timeout = timeout.as_secs_f64();
        let resp = tdjson_sys::td_json_client_receive(self.client_ptr, timeout);
        if resp.is_null() {
            return Ok(None);
        }
        let answer = CStr::from_ptr(resp);
        Ok(Some(answer.to_str().map_err(TdlibError::Utf8Error)?))
    }
}

impl Drop for UnsafeClient {
    fn drop(&mut self) {
        unsafe { tdjson_sys::td_json_client_destroy(self.client_ptr) }
    }
}

#[derive(Debug)]
pub struct Client {
    inner: UnsafeClient,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            inner: UnsafeClient::new(),
        }
    }
}

impl Client {
    /// Creates a new instance of TDLib.
    ///
    /// # Examples
    ///
    /// ```
    /// let client = Client::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Synchronously executes TDLib request.
    ///
    /// May be called from any thread. Only a few requests can be executed synchronously.
    ///
    /// # Examples
    ///
    /// ```
    /// let request = r#"{"@type": "getTextEntities", "text": "@telegram /test_command https://telegram.org telegram.me"}"#;
    /// client.execute(request);
    /// ```
    pub fn execute(&mut self, request: &str) -> Result<Option<&str>, TdlibError> {
        // SAFE: we are taking self by mutable reference.
        unsafe { self.inner.execute(request) }
    }

    #[cfg(feature = "types")]
    pub fn execute_typed<T>(&mut self, request: T) -> Result<Option<T::Response>, TdlibError>
    where
        T: tdlib_types::methods::Method,
    {
        let s = serde_json::to_string(&request.tag()).map_err(TdlibError::InvalidRequestData)?;
        match self.execute(&s) {
            Ok(ok) => match ok {
                Some(ok) => Ok(serde_json::from_str(ok).map_err(TdlibError::InvalidRequestData))?,
                None => Ok(None),
            },
            Err(e) => Err(e),
        }
    }

    /// Sends request to the TDLib client.
    ///
    /// May be called from any thread.
    ///
    /// # Examples
    ///
    /// ```
    /// let request = r#"{"@type": "getMe"}"#;
    /// client.send(request);
    /// ```
    pub fn send(&self, request: &str) -> Result<(), TdlibError> {
        self.inner.send(request)
    }

    #[cfg(feature = "types")]
    pub fn send_typed<T>(&self, request: T) -> Result<(), TdlibError>
    where
        T: tdlib_types::methods::Method,
    {
        let s = serde_json::to_string(&request.tag()).map_err(TdlibError::InvalidRequestData)?;
        self.send(&s)
    }

    /// Receives incoming updates and request responses from the TDLib client.
    ///
    /// May be called from any thread, but shouldn't be called simultaneously
    /// from two different threads.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// client.receive(Duration::from_secs(5));
    /// ```
    pub fn receive(&mut self, timeout: Duration) -> Result<Option<&str>, TdlibError> {
        // SAFE: we are taking self by mutable reference.
        unsafe { self.inner.receive(timeout) }
    }

    #[cfg(feature = "types")]
    pub fn receive_typed(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<tdlib_types::types::Response>, TdlibError> {
        match self.receive(timeout) {
            Ok(ok) => match ok {
                Some(ok) => Ok(serde_json::from_str(ok).map_err(TdlibError::InvalidRequestData))?,
                None => Ok(None),
            },
            Err(e) => Err(e),
        }
    }

    pub fn split(self) -> (SendClient, ReceiveClient) {
        let c = Arc::new(self.inner);
        let s = SendClient { inner: c.clone() };
        let r = ReceiveClient { inner: c };
        (s, r)
    }
}

#[derive(Debug)]
pub struct SendClient {
    inner: Arc<UnsafeClient>,
}

#[derive(Debug)]
pub struct ReceiveClient {
    inner: Arc<UnsafeClient>,
}

/// SAFE: the send method can be called by any thread.
unsafe impl Send for SendClient {}

/// SAFE: the send method can be called by multiple threads at the same time.
unsafe impl Sync for SendClient {}

impl SendClient {
    pub fn send(&self, request: &str) -> Result<(), TdlibError> {
        self.inner.send(request)
    }

    #[cfg(feature = "types")]
    pub fn send_typed<T>(&self, request: T) -> Result<(), TdlibError>
    where
        T: tdlib_types::methods::Method,
    {
        let s = serde_json::to_string(&request.tag()).map_err(TdlibError::InvalidRequestData)?;
        self.send(&s)
    }
}

/// SAFE: the receive method can be called by any thread.
unsafe impl Send for ReceiveClient {}

impl ReceiveClient {
    pub fn receive(&mut self, timeout: Duration) -> Result<Option<&str>, TdlibError> {
        // SAFE: we are taking self by mutable reference.
        unsafe { self.inner.receive(timeout) }
    }

    #[cfg(feature = "types")]
    pub fn receive_typed(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<tdlib_types::types::Response>, TdlibError> {
        match self.receive(timeout) {
            Ok(ok) => match ok {
                Some(ok) => Ok(serde_json::from_str(ok).map_err(TdlibError::InvalidRequestData))?,
                None => Ok(None),
            },
            Err(e) => Err(e),
        }
    }
}
