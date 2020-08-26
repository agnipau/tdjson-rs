use {
    crate::error::TdlibError,
    std::{
        ffi::{CStr, CString},
        os::raw::{c_char, c_void},
        time::Duration,
    },
};

#[derive(Debug)]
pub(crate) struct UnsafeClient {
    client_ptr: *mut c_void,
}

impl UnsafeClient {
    pub(crate) fn new() -> Self {
        UnsafeClient {
            client_ptr: unsafe { tdjson_sys::td_json_client_create() },
        }
    }

    /// UNSAFE: the returned slice is invalidated upon the next call to execute
    /// or receive.
    pub(crate) unsafe fn execute(&self, request: &str) -> Result<Option<&str>, TdlibError> {
        let crequest = CString::new(request).map_err(TdlibError::NulError)?;
        let answer = tdjson_sys::td_json_client_execute(self.client_ptr, crequest.as_ptr());
        let answer = answer as *const c_char;
        if answer.is_null() {
            return Ok(None);
        }
        let answer = CStr::from_ptr(answer);
        Ok(Some(answer.to_str().map_err(TdlibError::Utf8Error)?))
    }

    pub(crate) fn send(&self, request: &str) -> Result<(), TdlibError> {
        let crequest = CString::new(request).map_err(TdlibError::NulError)?;
        unsafe { tdjson_sys::td_json_client_send(self.client_ptr, crequest.as_ptr()) }
        Ok(())
    }

    /// UNSAFE: the returned slice is invalidated upon the next call to execute
    /// or receive.
    pub(crate) unsafe fn receive(&self, timeout: Duration) -> Result<Option<&str>, TdlibError> {
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
