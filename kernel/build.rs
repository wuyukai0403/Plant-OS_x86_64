fn main() {
    // Tell cargo to pass the linker script to the linker..
    println!("cargo:rustc-link-arg=-Tkernel/linker.ld");
    // ..and to re-run if it changes.
    println!("cargo:rerun-if-changed=kernel/linker.ld");
}
