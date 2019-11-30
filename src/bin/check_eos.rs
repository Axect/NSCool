extern crate peroxide;
use peroxide::*;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    let df = DataFrame::read_nc(&format!("data/EOS/{}.nc", args[1])).expect(&format!("Can't read {}.nc", args[1]));
    df.print();
}