use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .flag("-mavx2")
        .file("src/c/array_ops.c")
        .include("src/c")
        .compile("libarray_ops.a");

    let bindings = bindgen::Builder::default()
        .header("src/c/array_ops.h")
        .generate()
        .expect("Faild to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("array_ops.rs"))
        .expect("Couldn't write bindings!");
}
