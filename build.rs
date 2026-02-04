use std::process::Command;
use std::env;

const CFLAGS: &'static str = "-fPIE -Wall -O2 -DSEF_FILE_ACCESS=0 -DSEF_STRING=1 -DSEF_PROGRAMMING_TOOLS=0 -DSEF_MEMORY_ALLOCATION=1 -DSEF_ARG_AND_EXIT_CODE=0 -DSEF_RETURN_STACK=2000 -DSEF_DATA_STACK=2000 -DSEF_CONTROL_FLOW_STACK=25 -DSEF_FORTH_MEMORY_SIZE=40000000 -DSEF_PAD_SIZE=200 -DSEF_ABORT_STOP_FORTH=1 -DSEF_CASE_INSENSITIVE=0 -DSEF_LOG_LEVEL=2 -DSEF_LOG_OVER_STDERR=0 -DSEF_STACK_BOUND_CHECK=1 -DSEF_CATCH_SEGFAULT=1";

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}", manifest_dir);
    println!("cargo:rustc-link-lib=seforthdvz");

    println!("cargo:rerun-if-changed=src/cancellable_thread.c");
    cc::Build::new()
        .file("src/cancellable_thread.c")
        .compile("cancellable_thread");

    Command::new("make").current_dir("./SEForth").env("CFLAGS", CFLAGS).arg("libseforth.a").status().unwrap();
    Command::new("cp").args(&["./SEForth/libseforth.a", "./libseforthdvz.a", "-f"]).status().unwrap();
}

