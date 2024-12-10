#![allow(missing_docs)]
fn main() {
    println!("cargo:rebuild-if-changed=device.yaml");
}
