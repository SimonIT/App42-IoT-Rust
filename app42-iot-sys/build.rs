extern crate bindgen;

use std::env;
use std::path::PathBuf;
use bindgen::CargoCallbacks;

fn main() {
    // This is the directory where the `c` library is located.
    let libdir_path = PathBuf::from("App42_IoT_SDK/V_1.0/App42/")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");

    // This is the path to the `c` headers file.
    let headers_path = libdir_path.join("Common/App42API.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // This is the path to the intermediate object file for our library.
    let obj_path = libdir_path.join("Common/App42API.o");
    // This is the path to the static library file.
    let lib_path = libdir_path.join("Common/libApp42API.a");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());

    // Tell cargo to tell rustc to link our `hello` library. Cargo will
    // automatically know it must look for a `libhello.a` file.
    println!("cargo:rustc-link-lib=App42API");

    // Tell cargo to invalidate the built crate whenever the header changes.
    println!("cargo:rerun-if-changed={}", headers_path_str);

    let modules = libdir_path.read_dir().unwrap().filter(|e| e.is_ok())
        .flat_map(|e| {
            let path = e.unwrap().path();
            let path_str = path.to_str().unwrap().to_string();
            vec!["-I".to_string(), path_str]
        })
        .collect::<Vec<_>>();

    // Run `clang` to compile the `hello.c` file into a `hello.o` object file.
    // Unwrap if it is not possible to spawn the process.
    if !std::process::Command::new("clang++")
        .arg("-c")
        .arg("-o")
        .arg(&obj_path)
        .args(&modules)
        .arg(libdir_path.join("Common/App42API.cpp"))
        .output()
        .expect("could not spawn `clang`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not compile object file");
    }

    // Run `ar` to generate the `libhello.a` file from the `hello.o` file.
    // Unwrap if it is not possible to spawn the process.
    if !std::process::Command::new("ar")
        .arg("rcs")
        .arg(lib_path)
        .arg(obj_path)
        .output()
        .expect("could not spawn `ar`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not emit library file");
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(headers_path_str)
        .allowlist_file(headers_path_str)
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_args(modules)
        .clang_arg("-frtti")
        .clang_arg("-std=c++11")
        .clang_arg("-fsigned-char")
        .clang_arg("-fexceptions")
        .opaque_type("std::.*")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(CargoCallbacks::new()))
        .use_core()
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
