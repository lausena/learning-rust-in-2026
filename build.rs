fn main() {
    println!("cargo:rerun-if-changed=src/hello_world.c");
    cc::Build::new()
        .file("src/hello_world.c")
        .compile("hello_world");

    println!("cargo:rerun-if-changed=src/count_spaces.c");
    cc::Build::new()
        .file("src/count_spaces.c")
        .compile("count_spaces");
}
