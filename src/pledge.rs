use std::{
    ffi::{CStr, FromBytesWithNulError},
    fmt, io, ptr,
};

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

pub fn pledge_promises<P>(promises: P) -> Result<(), Error>
where
    P: AsRef<[u8]>,
{
    let promises = CStr::from_bytes_with_nul(promises.as_ref())?;
    match unsafe { crate::ffi::pledge(promises.as_ptr(), ptr::null()) } {
        0 => Ok(()),
        _ => Err(get_error()),
    }
}

pub fn pledge_execpromises<E>(execpromises: E) -> Result<(), Error>
where
    E: AsRef<[u8]>,
{
    let execpromises = CStr::from_bytes_with_nul(execpromises.as_ref())?;
    match unsafe { crate::ffi::pledge(ptr::null(), execpromises.as_ptr()) } {
        0 => Ok(()),
        _ => Err(get_error()),
    }
}

pub fn pledge<P, E>(promises: P, execpromises: E) -> Result<(), Error>
where
    P: AsRef<[u8]>,
    E: AsRef<[u8]>,
{
    let promises = CStr::from_bytes_with_nul(promises.as_ref())?;
    let execpromises = CStr::from_bytes_with_nul(execpromises.as_ref())?;
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
    Nul(FromBytesWithNulError),
    EPERM,
    EFAULT,
    EINVAL,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nul(error) => error.fmt(f),
            Self::EPERM => f.write_str("This process is attempting to increase permissions."),
            Self::EFAULT => f.write_str(
                "promises or execpromises points outside the process's allocated address space.",
            ),
            Self::EINVAL => f.write_str("promises is malformed or contains invalid keywords."),
        }
    }
}

impl std::error::Error for Error {}

impl From<FromBytesWithNulError> for Error {
    fn from(error: FromBytesWithNulError) -> Self {
        Self::Nul(error)
    }
}
