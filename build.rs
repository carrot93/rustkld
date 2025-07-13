extern crate bindgen;

use bindgen::Builder;
use std::path::PathBuf;

const FILEPATH: &str = "src/bindings.rs";

fn main() {
    let src_base = match std::env::var("SRC_BASE") {
        Ok(s) => format!("-I{s}/sys"),
        _ => "-I/usr/src/sys".to_string()
    };
    let bindings = Builder::default()
        .allowlist_function("uprintf")
        .allowlist_function("module_register")
        .allowlist_type("moduledata_t")
        .allowlist_type("module")
        .allowlist_type("modeventtype")
        .allowlist_var("MOD_LOAD")
        .allowlist_var("MOD_UNLOAD")
        .allowlist_var("EOPNOTSUPP")

        .use_core()
        .ctypes_prefix("libc")
        .size_t_is_usize(true)
        
        .header("wrapper.h")

        .clang_arg("-D_KERNEL")
        .clang_arg("-DKLD_MODULE")

        .clang_arg("-I.")
        .clang_arg(src_base)

        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(FILEPATH);
    bindings
        .write_to_file(out_path)
        .expect("Unable to write bindings");
}
