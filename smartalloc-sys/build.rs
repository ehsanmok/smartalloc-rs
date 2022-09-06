#![no_std]

fn main() {
    cc::Build::new()
        .include("csrc")
        .file("csrc/smartall.c")
        .define("SMARTALLOC", None)
        .compile("smartall");
}
