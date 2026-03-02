fn main() {
    println!("cargo:rerun-if-changed=src/count_spaces.c");
    cc::Build::new()
        .file("src/count_spaces.c")
        .compile("count_spaces");
}
