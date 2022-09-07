//! [![no std](https://img.shields.io/badge/no-std-red)](https://img.shields.io/badge/no-std-red)
//! [![crates.io](https://img.shields.io/crates/v/smartalloc.svg)](https://crates.io/crates/smartalloc)
//! [![docs.rs](https://docs.rs/smartalloc/badge.svg)](https://docs.rs/smartalloc)
//! [![GitHub](https://img.shields.io/crates/l/smartalloc)](https://github.com/ehsanmok/smartalloc-rs)
//!
//! <br>
//!
//! This crate provides a `no_std` idiomatic Rust binding to [smartalloc](https://www.fourmilab.ch/smartall/) used for
//! **detecting orphaned buffer allocation** which is a type of heap memory leak that the program has lost all access to it.
//! The primary usecase is as a *debugging* tool when writing **unsafe** code where normal Rust static checks are not available.
//! It is best used along side [SANs](https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html) where SANs
//! alone are either unable to detect or their outputs are cumbersome to work through.
//! To get the best experience, `RUSTFLAGS=-Zsanitizer=leak` is used and is included in `.cargo/config.toml`.
//!
//! <br>
//!
//! ## Usage
//!
//! ```ini
//! [dev-dependencies]
//! smartalloc = "0.2"
//! ```
//!
//! In fact, with `#![cfg(debug_assertions)]` the crate does **not** compile in the `--release` mode so preventing from any accidental usage.
//! The crate **requires nightly** Rust toolchain (MSRV 1.65).
//!
//! <br>
//!
//! ## Example
//!
//! During debugging, configure the `SmartAlloc` as the global allocator. Then include `sm_dump(true)` at the end of an unsafe code block.
//! Here is the [examples/orphan.rs](https://github.com/ehsanmok/smartalloc-rs/blob/main/examples/orphan.rs)
//!
//! ```no_run
//! use core::alloc::{GlobalAlloc, Layout};
//!
//! use smartalloc::{sm_dump, SmartAlloc};
//!
//! #[global_allocator]
//! static GLOBAL: SmartAlloc = SmartAlloc;
//!
//! fn main() {
//!     unsafe {
//!         let alloc = SmartAlloc;
//!         let layout = Layout::from_size_align(8, 8).unwrap();
//!         alloc.alloc(layout); // orphaned memory leak as it's pointer is lost
//!                              // and there's no alloc.dealloc(ptr, layout)
//!         sm_dump(true);
//!     }
//! }
//! ```
//!
//! which outputs
//!
//! ```txt
//! Orphaned buffer:       8 bytes allocated at line 12 of examples/orphan.rs
//! ```
//!
//! *Note* that the detector also throws
//!
//! ```txt
//! Orphaned buffer:       5 bytes allocated at line 5 of examples/orphan.rs
//! Orphaned buffer:      48 bytes allocated at line 5 of examples/orphan.rs
//! ```
//!
//! which refers to the `#[global_allocator]` itself and can be ignored.
//!
//! <br>
//!
//! ## Features
//!
//! The detector can be turned off using `sm_static(true)` and turned back on `sm_static(false)` to wrap cases where allocation
//! is done through std or safe cases such as [examples/native.rs](https://github.com/ehsanmok/smartalloc-rs/blob/main/examples/native.rs).
//! For more details, checkout the original [docs](https://www.fourmilab.ch/smartall/).
//!
//! ## Aren't SANs alone supposed to detect such errors?
//!
//! Neither of the `leak/address/memory` [sanitizers](https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html) are sufficient and can detect such errors *easily*.
//! In fact, running
//!
//! ```txt
//! RUSTFLAGS="-Zsanitizer=leak" cargo +nightly run --example undetected
//! // OR
//! RUSTFLAGS="-Zsanitizer=address" cargo +nightly run --example undetected
//! ```
//!
//! for [examples/undetected.rs](https://github.com/ehsanmok/smartalloc-rs/blob/main/examples/undetected.rs) which is
//!
//! ```no_run
//! unsafe {
//!     let alloc = SmartAlloc;
//!     let layout = Layout::from_size_align(8, 8).unwrap();
//!     alloc.alloc(layout);
//! }
//! ```
//!
//! with no `sm_dump(true)` at the end, does not show anything, mainly because we specified
//!
//! ```ini
//! [profile.dev]
//! opt-level = 0
//! ```
//!
//! for the SmartAlloc to work with introspection as opposed to what has been advised to include (at least `opt-level=1`)
//! [here](https://github.com/japaric/rust-san#unrealiable-leaksanitizer)
//! to cirvumvent such a limitation but when is done the context gets destroyed. Also
//!
//! ```txt
//! RUSTFLAGS="-Zsanitizer=memory -Zsanitizer-memory-track-origins" cargo +nightly run --example undetected
//! ```
//!
//! cannot compile and it throws unhelpful messages
//!
//! ```txt
//! error: failed to run custom build command for `libc v0.2.132`
//!
//! Caused by:
//!   process didn't exit successfully: `/home/workspace/smartalloc-rs/target/debug/build/libc-02d4e594eff5723f/build-script-build` (exit status: 1)
//!   --- stdout
//!   cargo:rerun-if-changed=build.rs
//!
//!   --- stderr
//!   ==186416==WARNING: MemorySanitizer: use-of-uninitialized-value
//!     #0 0x56367729226c  (/home/workspace/smartalloc-rs/target/debug/build/libc-02d4e594eff5723f/build-script-build+0x7a26c) (BuildId: ff090caba1904387acf3f0fecb58801c6fa5caed)
//!     #1 0x56367728e95d  (/home/workspace/smartalloc-rs/target/debug/build/libc-02d4e594eff5723f/build-script-build+0x7695d) (BuildId: ff090caba1904387acf3f0fecb58801c6fa5caed)
//!     ...
//!     Uninitialized value was created by an allocation of '_2' in the stack frame of function '_ZN18build_script_build19rustc_minor_nightly17hfbf53e202478a57bE'
//!       #0 0x563677291e70  (/home/workspace/smartalloc-rs/target/debug/build/libc-02d4e594eff5723f/build-script-build+0x79e70) (BuildId: ff090caba1904387acf3f0fecb58801c6fa5caed)
//!
//!     SUMMARY: MemorySanitizer: use-of-uninitialized-value (/home/workspace/smartalloc-rs/target/debug/build/libc-02d4e594eff5723f/build-script-build+0x7a26c) (BuildId: ff090caba1904387acf3f0fecb58801c6fa5caed)
//!     Exiting
//! ```
//!
//! so it needs more work!
//!
//! <br>
//!
//! ## Known issue
//!
//! [smartalloc-sys/csrc/smartall.c](https://github.com/ehsanmok/smartalloc-rs/blob/main/smartalloc-sys/csrc/smartall.c)
//! writes into the passed filename pointer tracked by `#[track_caller]` (which is immutable)
//! which is an UB that could result into displaying more garbage after the filename in its report using this binding.

#![no_std]
#![crate_type = "lib"]
#![cfg(debug_assertions)]

extern crate smartalloc_sys as ffi;

use ffi::{c_char, c_int, c_void};

use core::alloc::{GlobalAlloc, Layout};
use core::panic::Location;

/// Prints orphaned buffers when enabled `true` with
/// the number of allocated bytes and the line location in source code
pub fn sm_dump(enable: bool) {
    unsafe { ffi::sm_dump(enable.into()) }
}

/// Orphaned buffer detection can be disabled  (for  such
/// items  as buffers allocated during initialisation) by
/// calling `sm_static(true)`. Normal orphaned buffer
/// detection  can be re-enabled with `sm_static(false)`. Note
/// that all the other safeguards still apply to  buffers
/// allocated when `sm_static(true)` mode is in effect.
pub fn sm_static(enable: bool) {
    unsafe { ffi::sm_static(enable.into()) }
}

/// SmartAlloc allocator which needs to be use with `#[global_allocator]`
pub struct SmartAlloc;

unsafe impl GlobalAlloc for SmartAlloc {
    #[track_caller]
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let caller_loc = Location::caller();
        let fname = caller_loc.file();
        let lineno = caller_loc.line();
        ffi::sm_malloc(
            // this is an unfortunate UB making the output contain some garbage
            // and seems no other way to make it safe as
            // `smartall.c` writes into the passed ptr and `fname` is behind a shared pointer
            // and also neither of `core::cell::Cell` or `alloc::string::String.as_mut_ptr`
            // can guarentee UTF-8 validity concernes!
            fname.as_ptr() as *const _ as *mut c_char,
            lineno as c_int,
            layout.size(),
        ) as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ffi::sm_free(ptr as *mut c_void);
    }

    #[track_caller]
    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let caller_loc = Location::caller();
        let fname = caller_loc.file();
        let lineno = caller_loc.line();
        ffi::sm_calloc(
            // this is an unfortunate UB making the output contain some garbage
            // and seems no other way to make it safe as
            // `smartall.c` writes into the passed ptr and `fname` is behind a shared pointer
            // and also neither of `core::cell::Cell` or `alloc::string::String.as_mut_ptr`
            // can guarentee UTF-8 validity concernes!
            fname.as_ptr() as *const _ as *mut c_char,
            lineno as c_int,
            1,
            layout.size(),
        ) as *mut u8
    }

    #[track_caller]
    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        let caller_loc = Location::caller();
        let fname = caller_loc.file();
        let lineno = caller_loc.line();
        ffi::sm_realloc(
            // this is an unfortunate UB making the output contain some garbage
            // and seems no other way to make it safe as
            // `smartall.c` writes into the passed ptr and `fname` is behind a shared pointer
            // and also neither of `core::cell::Cell` or `alloc::string::String.as_mut_ptr`
            // can guarentee UTF-8 validity concernes!
            fname.as_ptr() as *const _ as *mut c_char,
            lineno as c_int,
            ptr as *mut c_void,
            new_size,
        ) as *mut u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn free_alloc() {
        unsafe {
            let layout = Layout::from_size_align(8, 8).unwrap();
            let alloc = SmartAlloc;
            let ptr = alloc.alloc(layout);
            alloc.dealloc(ptr, layout);
        }
        sm_dump(true);
    }

    #[test]
    fn free_big_alloc() {
        unsafe {
            let layout = Layout::from_size_align(1 << 20, 32).unwrap();
            let alloc = SmartAlloc;
            let ptr = alloc.alloc(layout);
            alloc.dealloc(ptr, layout);
            sm_dump(true)
        }
    }

    #[test]
    fn free_zero_alloc() {
        unsafe {
            let layout = Layout::from_size_align(8, 8).unwrap();
            let alloc = SmartAlloc;
            let ptr = alloc.alloc_zeroed(layout);
            alloc.dealloc(ptr, layout);
            sm_dump(true)
        }
    }

    #[test]
    fn free_zero_big_alloc() {
        unsafe {
            let layout = Layout::from_size_align(1 << 20, 32).unwrap();
            let alloc = SmartAlloc;
            let ptr = alloc.alloc_zeroed(layout);
            alloc.dealloc(ptr, layout);
            sm_dump(true)
        }
    }

    #[test]
    fn free_realloc() {
        unsafe {
            let layout = Layout::from_size_align(8, 8).unwrap();
            let alloc = SmartAlloc;
            let ptr = alloc.alloc(layout);
            let ptr = alloc.realloc(ptr, layout, 16);
            alloc.dealloc(ptr, layout);
            sm_dump(true)
        }
    }

    #[test]
    fn free_big_realloc() {
        unsafe {
            let layout = Layout::from_size_align(1 << 20, 32).unwrap();
            let alloc = SmartAlloc;
            let ptr = alloc.alloc(layout);
            let ptr = alloc.realloc(ptr, layout, 2 << 20);
            alloc.dealloc(ptr, layout);
            sm_dump(true)
        }
    }
}
