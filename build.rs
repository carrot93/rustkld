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

        .use_core() // no std
        .ctypes_prefix("libc") // refer to c types as libc::
        .size_t_is_usize(true) // make C's size_t -> Rust's usize
        
        .header("wrapper.h") // our wrapper

        .clang_arg("-D_KERNEL") // enable _Kernel block in headers
        .clang_arg("-DKLD_MODULE") // enable KLD module specific declarations

        .clang_arg("-I.") // makes our wrapper.h world in our crate root
        .clang_arg(src_base) // include our SRC

        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(FILEPATH);
    bindings
        .write_to_file(out_path)
        .expect("Unable to write bindings");
}
