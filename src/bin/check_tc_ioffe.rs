extern crate nscool;
use nscool::backend::tc_ioffe::*;

fn main() {
    (1 .. 11).for_each(
        |x| {
            let kf: f64 = x.into();
            println!("kf: {}, Tc: {}", kf, Tc_Ioffe_1p(kf));
            println!("kf: {}, Tc: {}", kf, Tc_Ioffe_2p(kf));
            println!("kf: {}, Tc: {}", kf, Tc_Ioffe_3p(kf));
            println!("kf: {}, Tc: {}", kf, Tc_Ioffe_1ns(kf));
            println!("kf: {}, Tc: {}", kf, Tc_Ioffe_2ns(kf));
            println!("kf: {}, Tc: {}", kf, Tc_Ioffe_3ns(kf));
            println!("kf: {}, Tc: {}", kf, Tc_Ioffe_1nt(kf));
            println!("kf: {}, Tc: {}", kf, Tc_Ioffe_2nt(kf));
            println!("kf: {}, Tc: {}", kf, Tc_Ioffe_3nt(kf));
        }
    )
}