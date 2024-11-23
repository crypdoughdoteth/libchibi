fn main() {
    println!("cargo::rerun-if-changed=src/chibihash64.c");
    cc::Build::new()
        .file("src/chibihash64.c")
        .compile("chibihash64");
}
