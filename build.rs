use prost_build;

fn main() {
    println!("cargo:rerun-if-changed=proto/format.proto");
    prost_build::compile_protos(&["proto/format.proto"], &["proto/"]).unwrap();
}
