extern crate rustfmt_nightly;
#[macro_use] extern crate quote;
extern crate svd2rust;

use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let xml = include_str!("./MK20D7.svd");
    let device = svd2rust::svd::parse(xml);
    let target = svd2rust::target::Target::CortexM;
    let items = svd2rust::generate::device::render(&device, &target).unwrap();

    let peripherals = quote! {
        #(#items)*
    };

    let lib = rustfmt_nightly::format_snippet(
        &peripherals.to_string(),
        &rustfmt_nightly::load_config(None, None).unwrap().0,
    ).unwrap();

    let out_path = PathBuf::from("./src/lib.rs");
    let mut f = File::create(out_path).unwrap();
    f.write_all(lib.as_bytes()).unwrap();
}
