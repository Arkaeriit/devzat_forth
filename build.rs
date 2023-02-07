fn main() {
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=amForth");

    /*
    println!("cargo:rerun-if-changed=src/set_limit.c");
    cc::Build::new()
        .file("src/set_limit.c")
        .compile("set_limit");
    */
}

