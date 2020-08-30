use {
    crate::{
        client::unsafe_::UnsafeClient,
        error::{Error, Result},
    },
    std::sync::Arc,
};

#[derive(Debug)]
pub struct SendClient {
    pub(crate) inner: Arc<UnsafeClient>,
}

/// SAFE: the send method can be called by any thread.
unsafe impl Send for SendClient {}

/// SAFE: the send method can be called by multiple threads at the same time.
unsafe impl Sync for SendClient {}

impl SendClient {
    pub fn send(&self, request: &str) -> Result<()> {
        self.inner.send(request)
    }

    #[cfg(feature = "types")]
    pub fn send_typed<T>(&self, request: T) -> Result<()>
    where
        T: tdlib_types::methods::Method,
    {
        let s = serde_json::to_string(&request.tag()).map_err(Error::InvalidRequestData)?;
        self.send(&s)
    }
}
