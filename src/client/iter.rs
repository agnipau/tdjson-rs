use {crate::client::safe::Client, std::time::Duration};

#[cfg(feature = "types")]
use tdlib_types::types::Response;

pub struct ClientIter {
    pub client: Client,
    timeout: Duration,
}

impl ClientIter {
    pub fn new(timeout: Duration) -> Self {
        Self {
            client: Client::new(),
            timeout,
        }
    }
}

impl Iterator for ClientIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Ok(resp) = self.client.receive(self.timeout) {
                if let Some(resp) = resp {
                    return Some(resp.into());
                }
            }
        }
    }
}

#[cfg(feature = "types")]
pub struct TypedClientIter {
    pub client: Client,
    timeout: Duration,
}

#[cfg(feature = "types")]
impl TypedClientIter {
    pub fn new(timeout: Duration) -> Self {
        Self {
            client: Client::new(),
            timeout,
        }
    }
}

#[cfg(feature = "types")]
impl Iterator for TypedClientIter {
    type Item = Response;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Ok(resp) = self.client.receive_typed(self.timeout) {
                if let Some(resp) = resp {
                    return Some(resp);
                }
            }
        }
    }
}
