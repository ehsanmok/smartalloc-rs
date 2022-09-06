#![no_std]

pub use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub enum Boolean {
    False = 0,
    True = 1,
}

impl From<bool> for Boolean {
    fn from(b: bool) -> Self {
        match b {
            false => Boolean::False,
            true => Boolean::True,
        }
    }
}

#[link(name = "smartall", kind = "static")]
extern "C" {
    pub fn sm_malloc(fname: *mut c_char, lineno: c_int, nbytes: usize) -> *mut c_void;
    pub fn sm_calloc(fname: *mut c_char, lineno: c_int, nelem: usize, elsize: usize)
        -> *mut c_void;
    pub fn sm_realloc(
        fname: *mut c_char,
        lineno: c_int,
        ptr: *mut c_void,
        size: usize,
    ) -> *mut c_void;
    pub fn sm_free(ptr: *mut c_void);
    pub fn sm_dump(datadump: Boolean);
    pub fn sm_static(mode: isize);
}
