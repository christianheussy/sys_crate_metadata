// build.rs

fn main() {
    let mut cfg = cc::Build::new();
    cfg.file("foo.c");
    let out_dir = std::env::var("OUT_DIR").unwrap();

    // get MANIFEST_DIR environment variable
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // copy header to OUT_DIR
    let header_path = format!("{}/foo.h", manifest_dir);

    // copy header to OUT_DIR
    std::fs::copy(header_path, format!("{out_dir}/foo.h")).unwrap();

    println!("cargo:include={out_dir}");
    cfg.compile("foo");
    println!("cargo:rerun-if-changed=foo.c");
}
