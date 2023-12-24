// build.rs

fn main() {
    let mut cfg = cc::Build::new();
    cfg.file("bar.c");

    // find include path
    // this is kinda the whole point of this exercise :)
    if let Some(include) = std::env::var_os("DEP_FOO_INCLUDE") {
        cfg.include(include);
    } else {
        println!("cargo:warning=Unable to locate includes.")
    }

    cfg.compile("bar");
    println!("cargo:rerun-if-changed=bar.c");
}
