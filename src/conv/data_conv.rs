use peroxide::*;

use std::error::Error;
//use crate::conv::data_type::CrustData;


pub fn crust_conv(data: &str) -> Result<(), Box<dyn Error>> {
    let df = DataFrame::read_csv(&format!("data/EOS/Crust/{}.csv", data), ',')?;
    df.write_nc(&format!("data/EOS/Crust/{}.nc", data))
}

pub fn eos_conv(data: &str) -> Result<(), Box<dyn Error>> {
    let df = DataFrame::read_csv(&format!("data/EOS/{}.csv", data), ',')?;
    df.write_nc(&format!("data/EOS/{}.nc", data))
}