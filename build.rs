extern crate svd2rust;
#[macro_use] extern crate quote;

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

    let out_path = PathBuf::from("./src/lib.rs");
    let mut f = File::create(out_path).unwrap();
    f.write_all(peripherals.to_string().as_bytes()).unwrap();
}
