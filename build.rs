fn main() {
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=amForth");

    println!("cargo:rerun-if-changed=src/cancellable_thread.c");
    cc::Build::new()
        .file("src/cancellable_thread.c")
        .compile("cancellable_thread");
}

