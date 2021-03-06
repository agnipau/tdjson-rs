#![cfg(feature = "types")]

use {
    crate::{
        client::{receive::ReceiveClient, send::SendClient, untyped::Client},
        error::{Error, Result},
    },
    std::time::Duration,
    tdlib_types::{methods::Method, types::Response},
};

#[derive(Debug)]
pub struct TypedClient {
    inner: Client,
    pub timeout: Duration,
}

impl TypedClient {
    /// Creates a new instance of TDLib.
    pub fn new(timeout: Duration) -> Self {
        Self {
            inner: Client::new(timeout),
            timeout,
        }
    }

    pub fn execute_untyped(&mut self, request: &str) -> Result<Option<&str>> {
        self.inner.execute(request)
    }

    /// Synchronously executes TDLib request.
    ///
    /// May be called from any thread. Only a few requests can be executed synchronously.
    pub fn execute<T>(&mut self, request: T) -> Result<Option<T::Response>>
    where
        T: Method,
    {
        let s = serde_json::to_string(&request.tag()).map_err(Error::InvalidRequestData)?;

        match self.execute_untyped(&s) {
            Ok(ok) => match ok {
                Some(ok) => Ok(serde_json::from_str(ok).map_err(Error::InvalidRequestData))?,
                None => Ok(None),
            },
            Err(e) => Err(e),
        }
    }

    pub fn send_untyped(&self, request: &str) -> Result<()> {
        self.inner.send(request)
    }

    /// Sends request to the TDLib client.
    ///
    /// May be called from any thread.
    pub fn send<T>(&self, request: T) -> Result<()>
    where
        T: Method,
    {
        let s = serde_json::to_string(&request.tag()).map_err(Error::InvalidRequestData)?;
        self.send_untyped(&s)
    }

    pub fn receive_untyped(&mut self, timeout: Duration) -> Result<Option<&str>> {
        self.inner.receive(timeout)
    }

    /// Receives incoming updates and request responses from the TDLib client.
    ///
    /// May be called from any thread, but shouldn't be called simultaneously
    /// from two different threads.
    pub fn receive(&mut self, timeout: Duration) -> Result<Option<Response>> {
        match self.receive_untyped(timeout) {
            Ok(ok) => match ok {
                Some(ok) => Ok(serde_json::from_str(ok).map_err(Error::InvalidRequestData))?,
                None => Ok(None),
            },
            Err(e) => Err(e),
        }
    }

    pub fn split(self) -> (SendClient, ReceiveClient) {
        self.inner.split()
    }
}

impl Iterator for TypedClient {
    type Item = Response;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Ok(resp) = self.receive(self.timeout) {
                if let Some(resp) = resp {
                    return Some(resp);
                }
            }
        }
    }
}
