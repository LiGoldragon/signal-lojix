use std::{env, fs, path::PathBuf};

use ethos_zero::{File, Generating};
use protos::{Actualizable, Potential};

fn main() {
    println!("cargo:rerun-if-changed=ethos/lib.ethos");
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory"));
    let source = fs::read_to_string(root.join("ethos/lib.ethos")).expect("read Signal source");
    let file = Potential::<File>::from(source).actualize(()).expect("parse checked Signal source");
    let generated = file.generate().expect("generate checked Signal Rust");
    fs::write(root.join("src/generated.rs"), generated).expect("write Signal Rust");
}
