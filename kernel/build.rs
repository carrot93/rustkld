extern crate bindgen;

use bindgen::{Builder, MacroTypeVariation::Signed};
use std::path::PathBuf;

const FILEPATH: &str = "src/bindings/imports.rs";
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

        // Allowed Functions
        .allowlist_function("uiomove*")

        .allowlist_function("make_dev_p*")
        .allowlist_function("destroy_dev")

        .allowlist_function("malloc")
        .allowlist_function("free")

        .allowlist_function("uprintf")

        .allowlist_function("dev_ref")
        .allowlist_function("dev_rel")
    
        // Allowed Types
        .allowlist_type("uio")

        .allowlist_type("cdev")
        .allowlist_type("cdevsw")
        
        .allowlist_type("moduledata_t")

        // Allowed Variables
        .allowlist_var("M_DEVBUF")
        .allowlist_var("M_WAITOK")

        .allowlist_var("MAKEDEV_WAITOK")
        .allowlist_var("MAKEDEV_CHECKNAME")

        .allowlist_var("UID_ROOT")
        .allowlist_var("GID_WHEEL")

        .allowlist_var("D_VERSION")

        // End of allowlist

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
