use std::process::Command;
use std::env;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}", manifest_dir);
    println!("cargo:rustc-link-lib=seforth");

    println!("cargo:rerun-if-changed=src/cancellable_thread.c");
    cc::Build::new()
        .file("src/cancellable_thread.c")
        .compile("cancellable_thread");

    Command::new("git").args(&["clone", "https://github.com/Arkaeriit/SEForth.git"]).status().unwrap();
    Command::new("sed").args(&["-i", "SEForth/sef_config.h", "-e", "s:SEF_FILE.*:SEF_FILE 0:"]).status().unwrap(); // Yes, I use GNU sed syntax, this project is too stupid for me to care about portability
    Command::new("sed").args(&["-i", "SEForth/sef_config.h", "-e", "s:SEF_PROGRAMMING_TOOLS.*:SEF_PROGRAMMING_TOOLS 0:"]).status().unwrap();
    Command::new("make").current_dir("./SEForth").env("CFLAGS", "-fPIE").arg("libseforth.a").status().unwrap();
    Command::new("cp").args(&["./SEForth/libseforth.a", "./", "-f"]).status().unwrap();
}

