use {
    crate::{
        client::{receive::ReceiveClient, send::SendClient, unsafe_::UnsafeClient},
        error::TdlibError,
    },
    std::{sync::Arc, time::Duration},
};

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
