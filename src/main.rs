#![allow(non_snake_case)]
use d1::day_1;
use std::env;
use utils::check_for_file;
pub mod d1;
pub mod d2;
pub mod utils;

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    check_for_file("2");
    // day_1().unwrap()
}
