extern crate nscool;
use nscool::backend::density::density;

fn main() {
    let mut empty = Vec::new();
    println!("{:E}", density(1., 1., 1., 1., 1., &mut empty));
    println!("{:?}", empty);
}