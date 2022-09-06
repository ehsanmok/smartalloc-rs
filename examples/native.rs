use smartalloc::{sm_dump, sm_static, SmartAlloc};

#[global_allocator]
static GLOBAL: SmartAlloc = SmartAlloc;

fn main() {
    sm_static(true);
    let size = 50 * 1024 * 1024;
    let mut x: Vec<usize> = Vec::new();
    for i in 0..size {
        x.push(i);
    }
    println!("{:?}", x.iter().sum::<usize>());
    sm_static(false);
    sm_dump(true);
}
