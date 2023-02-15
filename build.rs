use std::process::Command;

fn main() {
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=amforth");

    println!("cargo:rerun-if-changed=src/cancellable_thread.c");
    cc::Build::new()
        .file("src/cancellable_thread.c")
        .compile("cancellable_thread");

    Command::new("git").args(&["clone", "https://github.com/Arkaeriit/ASCminiForth.git"]).status().unwrap();
    Command::new("sed").args(&["-i", "ASCminiForth/amf_config.h", "-e", "s:AMF_FILE.*:AMF_FILE 0:"]).status().unwrap(); // Yes, I use GNU sed syntax, this project is too stupid for me to care about portability
    Command::new("sed").args(&["-i", "ASCminiForth/amf_config.h", "-e", "s:AMF_PROGRAMMING_TOOLS.*:AMF_PROGRAMMING_TOOLS 0:"]).status().unwrap();
    Command::new("make").current_dir("./ASCminiForth").arg("libamforth.a").status().unwrap();
    Command::new("cp").args(&["./ASCminiForth/libamforth.a", "./", "-f"]).status().unwrap();
}

