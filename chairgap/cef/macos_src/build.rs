fn main() {
    println!("cargo:rerun-if-changed=src/application.mm");
    println!("cargo:rustc-link-lib=framework=AppKit");
    cc::Build::new()
        .cpp(true)
        .warnings(false)
        .flag("-Wno-deprecated-declarations")
        .flag("-std=c++11")
        .flag("-fobjc-arc")
        .flag("-fblocks")
        .include("../headers/mac")
        .file("src/application.mm")
        .compile("rs_cef_src_mac");
}
