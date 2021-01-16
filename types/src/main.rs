extern crate unicode_segmentation;


use std::string::String;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut s = String::from("hello world!");

    s.push_str(" world");

    println!("{}", s);
    match s.grapheme_indices(true).nth(3) {
        Some((nsize, c)) => println!("{} {}", c, nsize),
        None => println!("NONE"),
    }
}
