use std::process::Command;

fn main() {
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=amForth");

    println!("cargo:rerun-if-changed=src/cancellable_thread.c");
    cc::Build::new()
        .file("src/cancellable_thread.c")
        .compile("cancellable_thread");

    Command::new("git").args(&["clone", "https://github.com/Arkaeriit/ASCminiForth.git"]).status().unwrap();
    Command::new("make").current_dir("./ASCminiForth").arg("libamForth.a").status().unwrap();
    Command::new("cp").args(&["./ASCminiForth/libamForth.a", "./", "-f"]).status().unwrap();
}

