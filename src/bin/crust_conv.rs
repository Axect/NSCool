extern crate nscool;
use nscool::conv::data_conv::*;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    crust_conv(&args[1]).expect(&format!("Can't convert {}", args[1]));
}