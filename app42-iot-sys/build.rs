#![feature(iter_intersperse)]

use std::env;

fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("App42_IoT_SDK/V_1.0/App42");

    let modules = path.read_dir()
        .unwrap()
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap().path().canonicalize().unwrap().to_str().unwrap().to_string())
        .intersperse(":".to_string())
        .collect::<String>();

    env::set_var("CPLUS_INCLUDE_PATH", format!("{}:{}", modules, env::var("CPLUS_INCLUDE_PATH").unwrap_or_default()));

    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&path]).build()?;
    b.flag_if_supported("-frtti")
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-fsigned-char")
        .flag_if_supported("-fexceptions")
        .file(path.join("Common/App42API.cpp"))
        .compile("autocxx-app42-iot");
    println!("cargo:rerun-if-changed=src/lib.rs");
    Ok(())
}
