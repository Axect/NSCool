extern crate nscool;
use nscool::backend::tc::*;

fn main() {
    println!("{}", tcn1_sfb(1f64));
    println!("{}", tcn1_ccdk(1f64));
    println!("{}", tcn1_wap(1f64));
    println!("{}", tcn1_gc(1f64));
    println!("{}", tcn1_gipsf(1f64));
    println!("{}", tcn1_t72(1f64));
    println!("{}", tcn1_ns(1f64));
    println!("{}", tcn1_t84(1f64));
    println!("{}", tcn1_ao(1f64));
    println!("{}", tcn1_ccks_var(1f64));
    println!("{}", tcn1_ccks_cbf(1f64));
    println!("{}", tcn1_awp_2(1f64));
    println!("{}", tcn1_awp_3(1f64));
//    println!("{}", tcn1_bbllp(1f64));
    println!("{}", tcn1_sclbl96(1f64));
    println!("{}", tcn1_sclbl96_pol(1f64));
    println!("{}", tcn3_hgrr(1f64));
    println!("{}", tcn3_ao(1f64));
    println!("{}", tcn3_t72(1f64));
    println!("{}", tcn3_t72_m1(1f64));
    println!("{}", tcn3_ao_m1(1f64));
    println!("{}", tcn3_bcll92(1f64));
    println!("{}", tcn3_eehjo96_nr(1f64));
    println!("{}", tcn3_eehjo96_r(1f64));
    println!("{}", tcp1_ccy_ms(1f64));
    println!("{}", tcp1_ccy_ps(1f64));
    println!("{}", tcp1_ccdk(1f64));
    println!("{}", tcp1_t73(1f64));
    println!("{}", tcp1_ns(1f64));
    println!("{}", tcp1_ao(1f64));
    println!("{}", tcp1_bcll92(1f64));
    println!("{}", tcp1_eeho(1f64));
}