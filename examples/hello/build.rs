extern crate fasters;
use fasters::{codegen, Dictionary, Version};
use std::fs;

const PATH: &str = "src/generated.rs";

fn main() {
    fs::File::create(PATH).unwrap();
    let dictionary = Dictionary::new(Version::Fix44);
    let code = codegen(dictionary);
    fs::write(PATH, code).unwrap();
}