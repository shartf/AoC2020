#![allow(non_snake_case)]
use d1::day_1;
use std::env;
use utils::check_for_file;
pub mod d1;
pub mod utils;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    check_for_file("1");
    day_1().unwrap()
}
