use bindgen::EnumVariation;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/bindings_mac.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
    if !Path::new("src/bindings_mac.rs").exists() {
        let bindings = bindgen::Builder::default()
            .default_enum_style(EnumVariation::Rust {
                non_exhaustive: true,
            })
            .dynamic_library_name("CefLibrary")
            .derive_default(true)
            .whitelist_function("cef_.*")
            .bitfield_enum(".*_mask_t")
            .header("wrapper.h")
            .clang_arg("-I../headers/mac")
            .generate()
            .expect("Unable to generate bindings");

        // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file("src/bindings_mac.rs")
            .expect("Couldn't write bindings!");
    }
}
