// Only used for compiling benchmarks
fn main() {
    println!("cargo:rustc-link-search=./benches/bindings/bin");
}