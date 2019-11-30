extern crate nscool;
use nscool::backend::density::P_electron;

fn main() {
    let mut empty_vec = vec![];
    println!("{}", P_electron(2f64, 1f64, &mut empty_vec));
    println!("{:?}", empty_vec);
    println!("{}", P_electron(2f64, 1f64, &mut empty_vec));
    println!("{:?}", empty_vec);
}