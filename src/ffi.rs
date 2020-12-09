use std::os::raw::{c_char, c_int};

extern "C" {
    pub fn pledge(promises: *const c_char, execpromises: *const c_char) -> c_int;
    pub fn unveil(path: *const c_char, permissions: *const c_char) -> c_int;
}
