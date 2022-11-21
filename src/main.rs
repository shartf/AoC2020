#![allow(non_snake_case)]
use std::env;
use utils::check_for_file;
pub mod utils;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    check_for_file("1");
}
