use std::env;
use std::path::PathBuf;

fn main() {
    // println!("cargo:rerun-if-changed=src/application.mm");
    // println!("cargo:rustc-link-lib=framework=AppKit");
    cc::Build::new()
        .cpp(true)
        .warnings(false)
        .flag("-Wno-deprecated-declarations")
        .flag("-std=gnu++11")
        .flag("-fno-exceptions")
        .flag("-fno-rtti")
        .flag("-fobjc-arc")
        .flag("-fblocks")
        .include("../wrapper/src")
        .file("src/application.mm")
        .file("src/browser.cc")
        .file("src/init.cc")
        .compile("chairgap_cef_binding");

    let bindings = bindgen::Builder::default()
        .header("src/index.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("binding.rs"))
        .expect("Couldn't write bindings!");
}
