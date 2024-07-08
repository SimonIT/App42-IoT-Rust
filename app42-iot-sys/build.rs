#![feature(iter_intersperse)]

use std::env;

fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("App42_IoT_SDK/V_1.0/App42");

    let modules = env::join_paths(
        path.read_dir()
        .unwrap()
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap().path().canonicalize().unwrap().into_os_string())
    ).unwrap();

    unsafe { env::set_var("CPLUS_INCLUDE_PATH", modules); }

    let mut b = autocxx_build::Builder::new("src/lib.rs", [&path]).build()?;
    b.flag_if_supported("-frtti")
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-fsigned-char")
        .flag_if_supported("-fexceptions")
        .file(path.join("Common/App42API.cpp"))
        .compile("autocxx-app42-iot");
    println!("cargo:rerun-if-changed={}", path.display());
    println!("cargo:rerun-if-changed=src/lib.rs");
    Ok(())
}
