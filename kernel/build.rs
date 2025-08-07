use bindgen::{Builder, MacroTypeVariation::Signed};
use std::{env, path::PathBuf};

const HEADERPATH: &str = "headers/wrapper.h";

fn main() {
    let src_base = match std::env::var("SRC_BASE") {
        Ok(s) => format!("-I{s}/sys"),
        _ => "-I/usr/src/sys".to_string(),
    };
    let bindings = Builder::default()
        .use_core()
        .ctypes_prefix("libc")
        .size_t_is_usize(true)
        .default_macro_constant_type(Signed)
        .header(HEADERPATH)
        .clang_arg("-D_KERNEL")
        .clang_arg("-DKLD_MODULE")
        .clang_arg("-I.")
        .clang_arg(src_base)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("imports.rs");
    bindings
        .write_to_file(out_path)
        .expect("Unable to write bindings");
}
