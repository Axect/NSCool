extern crate nscool;
use nscool::backend::density::pressure;

fn main() {
    let mut empty_set = Vec::new();
    println!("{:E}", pressure(1f64, 1f64, 1f64, 1f64, &mut empty_set));
    println!("{:?}", empty_set);
}