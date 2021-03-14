use bindgen::callbacks::EnumVariantValue;
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
struct ParseCallbacks {}

impl bindgen::callbacks::ParseCallbacks for ParseCallbacks {
    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: EnumVariantValue,
    ) -> Option<String> {
        if let Some(enum_name) = enum_name {
            if original_variant_name.starts_with(enum_name) {
                return Some(original_variant_name[enum_name.len()..].to_owned());
            } else {
                panic!(
                    "Enum variant {} doesn't start with its enum name: {}",
                    original_variant_name, enum_name
                )
            }
        } else {
            panic!(
                "Enum variant {} doesn't have an enum name",
                original_variant_name
            )
        }
    }
}

fn main() {
    println!("cargo:rustc-link-lib=framework=AppKit");
    println!("cargo:rustc-link-lib=framework=WebKit");
    cc::Build::new()
        .flag("-fobjc-arc")
        .flag("-fblocks")
        .file("src/app.m")
        .file("src/DGWindowController.m")
        .file("src/web_view.m")
        .compile("chairgap_darwin_src");

    let bindings = bindgen::Builder::default()
        .clang_args(&["-x", "objective-c"])
        .objc_extern_crate(true)
        .clang_arg("-fblocks")
        .generate_block(true)
        .block_extern_crate(true)
        .newtype_enum(".*")
        .parse_callbacks(Box::new(ParseCallbacks {}))
        .header("src/index.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
