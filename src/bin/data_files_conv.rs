extern crate peroxide;
use peroxide::*;

fn main() {
    let a = DataFrame::read_csv("data/Data_Files/con_crust_cryst_Itoh.csv", ',').expect("Can't read csv");
    a.write_nc("data/Data_Files/con_crust_cryst_Itoh.nc").expect("Can't write nc");
}