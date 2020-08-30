use {
    crate::{
        client::{receive::ReceiveClient, send::SendClient, unsafe_::UnsafeClient},
        error::Result,
    },
    std::{sync::Arc, time::Duration},
};

#[derive(Debug)]
pub struct Client {
    inner: UnsafeClient,
    pub timeout: Duration,
}

impl Client {
    /// Creates a new instance of TDLib.
    pub fn new(timeout: Duration) -> Self {
        Self {
            inner: UnsafeClient::new(),
            timeout,
        }
    }

    /// Synchronously executes TDLib request.
    ///
    /// May be called from any thread. Only a few requests can be executed synchronously.
    pub fn execute(&mut self, request: &str) -> Result<Option<&str>> {
        // SAFE: we are taking self by mutable reference.
        unsafe { self.inner.execute(request) }
    }

    /// Sends request to the TDLib client.
    ///
    /// May be called from any thread.
    pub fn send(&self, request: &str) -> Result<()> {
        self.inner.send(request)
    }

    /// Receives incoming updates and request responses from the TDLib client.
    ///
    /// May be called from any thread, but shouldn't be called simultaneously
    /// from two different threads.
    pub fn receive(&mut self, timeout: Duration) -> Result<Option<&str>> {
        // SAFE: we are taking self by mutable reference.
        unsafe { self.inner.receive(timeout) }
    }

    pub fn split(self) -> (SendClient, ReceiveClient) {
        let c = Arc::new(self.inner);
        let s = SendClient { inner: c.clone() };
        let r = ReceiveClient { inner: c };
        (s, r)
    }
}

impl Iterator for Client {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Ok(resp) = self.receive(self.timeout) {
                if let Some(resp) = resp {
                    return Some(resp.into());
                }
            }
        }
    }
}
