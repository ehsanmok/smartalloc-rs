use smartalloc::{sm_dump, sm_static, SmartAlloc};

#[global_allocator]
static GLOBAL: SmartAlloc = SmartAlloc;

fn main() {
    sm_static(true); // disable from here
    let size = 50 * 1024 * 1024;
    let mut x: Vec<usize> = Vec::new();
    for i in 0..size {
        x.push(i);
    }
    println!("{:?}", x.iter().sum::<usize>());
    sm_static(false); // enable from here
    sm_dump(true); // doesn't track anything after is enabled
}
