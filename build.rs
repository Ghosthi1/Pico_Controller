fn main() {
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=rp235x.x");
    println!("cargo:rustc-link-search=.");
}