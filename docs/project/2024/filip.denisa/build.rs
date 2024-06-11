fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
}
