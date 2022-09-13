use std::{
    ffi::{CStr, FromBytesWithNulError},
    fmt, io,
    os::unix::ffi::OsStrExt,
    path::Path,
    ptr,
};

pub fn unveil<P, X>(path: P, permissions: X) -> Result<(), Error>
where
    P: AsRef<Path>,
    X: AsRef<[u8]>,
{
    let path = CStr::from_bytes_with_nul(path.as_ref().as_os_str().as_bytes())?;
    let permissions = CStr::from_bytes_with_nul(permissions.as_ref())?;
    match unsafe { crate::ffi::unveil(path.as_ptr(), permissions.as_ptr()) } {
        0 => Ok(()),
        _ => Err(get_error()),
    }
}

/// Disables further calls to `unveil`.
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
    Nul(FromBytesWithNulError),
    EPERM,
    ENOENT,
    E2BIG,
    EINVAL,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nul(error) => error.fmt(f),
            Self::EPERM => f.write_str("An attempt to increase permissions was made, or the path was not accessible, or unveil() was called after locking."),
            Self::ENOENT => f.write_str("A directory in path did not exist."),
            Self::E2BIG => f.write_str("The addition of path would exceed the per-process limit for unveiled paths."),
            Self::EINVAL => f.write_str("An invalid value of permissions was used."),
        }
    }
}

impl std::error::Error for Error {}

impl From<FromBytesWithNulError> for Error {
    fn from(error: FromBytesWithNulError) -> Self {
        Self::Nul(error)
    }
}
