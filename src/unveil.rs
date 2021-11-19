use std::ffi::{CString, NulError};
use std::fmt;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr;

#[macro_export]
macro_rules! unveil {
    () => {
        crate::unveil::disable()
    };
    ($path:expr, $perm:expr $(,)?) => {
        crate::unveil::unveil($path, $perm)
    };
}

pub fn unveil(path: impl AsRef<Path>, permissions: impl Into<Vec<u8>>) -> Result<(), Error> {
    let path = CString::new(path.as_ref().as_os_str().as_bytes())?;
    let permissions = CString::new(permissions)?;
    match unsafe { crate::ffi::unveil(path.as_ptr(), permissions.as_ptr()) } {
        0 => Ok(()),
        _ => Err(get_error()),
    }
}

pub fn disable() {
    assert_eq!(unsafe { crate::ffi::unveil(ptr::null(), ptr::null()) }, 0);
}

fn get_error() -> Error {
    match io::Error::last_os_error().raw_os_error() {
        Some(1) => Error::EPERM,
        Some(2) => Error::ENOENT,
        Some(7) => Error::E2BIG,
        Some(22) => Error::EINVAL,
        e => unreachable!("Unexpected errno: {:?}", e),
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    NUL(NulError),
    EPERM,
    ENOENT,
    E2BIG,
    EINVAL,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NUL(e) => write!(f, "{}", e),
            Self::EPERM => write!(f, "An attempt to increase permissions was made, or the path was not accessible, or unveil() was called after locking."),
            Self::ENOENT => write!(f, "A directory in path did not exist."),
            Self::E2BIG => write!(f, "The addition of path would exceed the per-process limit for unveiled paths."),
            Self::EINVAL => write!(f, "An invalid value of permissions was used."),
        }
    }
}

impl std::error::Error for Error {}

impl From<NulError> for Error {
    fn from(e: NulError) -> Self {
        Self::NUL(e)
    }
}
