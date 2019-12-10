extern crate peroxide;

pub const HB: f64 = 1.054588E-27;
pub const KB: f64 = 1.380662E-16;
pub const NA: f64 = 6.022045E+23;
pub const C: f64 = 2.997924E+10;
pub const ME: f64 = 9.109E-28;

// Correspond to size.inc.f90
pub const ISIZE: usize = 10000;



pub mod conv;
pub mod backend;