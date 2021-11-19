use std::ffi::{CString, NulError};
use std::fmt;
use std::io;
use std::ptr;

#[macro_export]
macro_rules! pledge {
    ($p:expr $(,)?) => {
        crate::pledge::pledge_promises($p)
    };
    (_, $e:expr $(,)?) => {
        crate::pledge::pledge_execpromises($e)
    };
    ($p:expr, $e:expr $(,)?) => {
        crate::pledge::pledge($p, $e)
    };
}

pub fn pledge_promises(promises: impl Into<Vec<u8>>) -> Result<(), Error> {
    let promises = CString::new(promises)?;
    match unsafe { crate::ffi::pledge(promises.as_ptr(), ptr::null()) } {
        0 => Ok(()),
        _ => Err(get_error()),
    }
}

pub fn pledge_execpromises(execpromises: impl Into<Vec<u8>>) -> Result<(), Error> {
    let execpromises = CString::new(execpromises)?;
    match unsafe { crate::ffi::pledge(ptr::null(), execpromises.as_ptr()) } {
        0 => Ok(()),
        _ => Err(get_error()),
    }
}

pub fn pledge(promises: impl Into<Vec<u8>>, execpromises: impl Into<Vec<u8>>) -> Result<(), Error> {
    let promises = CString::new(promises)?;
    let execpromises = CString::new(execpromises)?;
    match unsafe { crate::ffi::pledge(promises.as_ptr(), execpromises.as_ptr()) } {
        0 => Ok(()),
        _ => Err(get_error()),
    }
}

fn get_error() -> Error {
    match io::Error::last_os_error().raw_os_error() {
        Some(1) => Error::EPERM,
        Some(14) => Error::EFAULT,
        Some(22) => Error::EINVAL,
        e => unreachable!("Unexpected errno: {:?}", e),
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    NUL(NulError),
    EPERM,
    EFAULT,
    EINVAL,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NUL(e) => write!(f, "{}", e),
            Self::EPERM => write!(f, "This process is attempting to increase permissions."),
            Self::EFAULT => write!(
                f,
                "promises or execpromises points outside the process's allocated address space."
            ),
            Self::EINVAL => write!(f, "promises is malformed or contains invalid keywords."),
        }
    }
}

impl std::error::Error for Error {}

impl From<NulError> for Error {
    fn from(e: NulError) -> Self {
        Self::NUL(e)
    }
}
