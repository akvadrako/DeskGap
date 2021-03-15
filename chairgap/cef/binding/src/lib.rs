#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/binding.rs"));

pub fn dummy_fn() {
    chairgap_cef_wrapper::dummy_fn()
}
