extern crate nscool;
use nscool::backend::eff_mass::*;

fn main() {
    (1..4).for_each(|x| println!("{:.10}", mstn_1(MSTN6::MSTN_AWP_3, x as f64)));
}