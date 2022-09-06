use core::alloc::{GlobalAlloc, Layout};

use smartalloc::SmartAlloc;

#[global_allocator]
static GLOBAL: SmartAlloc = SmartAlloc;

fn main() {
    unsafe {
        let alloc = SmartAlloc;
        let layout = Layout::from_size_align(8, 8).unwrap();
        alloc.alloc(layout);
    }
}
