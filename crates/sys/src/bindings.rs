#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    clippy::missing_safety_doc,
    clippy::transmute_int_to_bool,
    clippy::useless_transmute,
    rustdoc::broken_intra_doc_links,
// https://github.com/rust-lang/rust-bindgen/issues/2807
    unnecessary_transmutes,
// https://github.com/rust-lang/rust-bindgen/issues/3053
    clippy::ptr_offset_with_cast,
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
