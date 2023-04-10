extern crate bindgen;

use std::env;
use std::path::PathBuf;

use bindgen::CargoCallbacks;

fn main() {
    // This is the directory where the `c` library is located.
    let libdir_path = PathBuf::from("cpp_lib")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");

    // This is the path to the `c` headers file.
    let types_headers_path = libdir_path.join("include/types.h");
    let types_headers_path_str = types_headers_path
        .to_str()
        .expect("Path is not a valid string");

    let functions_headers_path = libdir_path.join("include/functions.h");
    let functions_headers_path_str = functions_headers_path
        .to_str()
        .expect("Path is not a valid string");
    let export_headers_path = libdir_path.join("include/cpp_lib_export.h");
    let export_headers_path_str = export_headers_path
        .to_str()
        .expect("Path is not a valid string");

    // Tell cargo to look for shared libraries in the specified directory
    println!(
        "cargo:rustc-link-search={}",
        libdir_path.join("lib/").to_str().unwrap()
    );

    // Tell cargo to tell rustc to link our `hello` library. Cargo will
    // automatically know it must look for a `libhello.a` file.
    println!("cargo:rustc-link-lib=cpp_lib");

    // Tell cargo to invalidate the built crate whenever the header changes.
    println!("cargo:rerun-if-changed={}", types_headers_path_str);
    println!("cargo:rerun-if-changed={}", functions_headers_path_str);
    println!("cargo:rerun-if-changed={}", export_headers_path_str);
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(types_headers_path_str)
        .header(functions_headers_path_str)
        .header(export_headers_path_str)
        .clang_arg("-Icpp_lib/include/")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
