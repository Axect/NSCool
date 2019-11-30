extern crate nscool;
use nscool::backend::density::P_electron;

fn main() {
    let mut empty_vec = vec![];
    println!("{:E}", P_electron(1f64, 6.022045E+23, &mut empty_vec));
    println!("{:?}", empty_vec);
    println!("{:E}", P_electron(1f64, 6.022045E+23, &mut empty_vec));
    println!("{:?}", empty_vec);
}