extern crate peroxide;
use peroxide::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Custom netcdf file
    let df = DataFrame::read_nc("data/Data_Files/con_crust_cryst_Itoh.nc")?;
    df.mean().print();

    Ok(())
}