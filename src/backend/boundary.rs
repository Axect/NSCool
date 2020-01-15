use std::os::raw::{c_double, c_int};
use crate::backend::inc::{Gravity};
use peroxide::{DataFrame, WithNetCDF};

#[allow(non_snake_case)]
#[link(name = "boundary")]
extern {
    pub fn fteff_zarho_(Tb: *const c_double, Z: *const c_double, A: *const c_double, Rho: *const c_double, gs14: *const c_double) -> c_double;
    pub fn fteff_nt_(Tb: *const c_double, gs14: *const c_double) -> c_double;
    pub fn fteff_gpe_(Tb: *const c_double, gs14: *const c_double) -> c_double;
    pub fn fteff_acc_(Tb: *const c_double, eta: *const c_double, gs14: *const c_double) -> c_double;
    pub fn fteff_field_iron_(Tb: *const c_double, bfield: *const c_double, gs14: *const c_double) -> c_double;
    pub fn splint1_(XA: *const c_double, YA: *const c_double, Y2A: *const c_double, i_n: *const c_int, X: *const c_double, Y: *mut c_double);
    pub fn spline1_(X: *const c_double, Y: *const c_double, i_n: *const c_int, YP1: *const c_double, YPN: *const c_double, Y2: *mut c_double);
}

#[allow(non_snake_case)]
pub fn fteff(Tb: f64, ifteff: usize, eta: f64, bfield: f64, istep: usize, time: f64, Ts1: f64, Ts2: f64, Z: f64, A: f64, Rho: f64, debug: f64, grav: &Gravity, bound: &mut Bound) -> f64 {
    if debug >= 1f64 {
        println!("Entering fteff");
    }
    let result = match ifteff {
//        0 => {
//            //fteff_table(Tb, Ts1, Ts2, grav, bound)
//            unimplemented!()
//        },
        1 => fteff_GPE(Tb, grav),
        2 => fteff_NT(Tb, grav),
        3 => fteff_acc(Tb, eta, grav),
        4 => fteff_field_iron(Tb, bfield, grav),
        5 => fteff_ZARho(Tb, Z, A, Rho, grav),
        _ => unreachable!(),
    };

    if debug >= 1f64 {
        println!("Done");
    }

    result
}

#[allow(non_snake_case)]
pub fn fteff_ZARho(Tb: f64, Z: f64, A: f64, Rho: f64, grav: &Gravity) -> f64 {
    unsafe {
        fteff_zarho_(&Tb, &Z, &A, &Rho, &grav.gs14)
    }
}

#[allow(non_snake_case)]
pub fn fteff_NT(Tb: f64, grav: &Gravity) -> f64 {
    unsafe {
        fteff_nt_(&Tb, &grav.gs14)
    }
}

#[allow(non_snake_case)]
pub fn fteff_GPE(Tb: f64, grav: &Gravity) -> f64 {
    unsafe {
        fteff_gpe_(&Tb, &grav.gs14)
    }
}

#[allow(non_snake_case)]
pub fn fteff_acc(Tb: f64, eta: f64, grav: &Gravity) -> f64 {
    unsafe {
        fteff_acc_(&Tb, &eta, &grav.gs14)
    }
}

#[allow(non_snake_case)]
pub fn fteff_field_iron(Tb: f64, bfield: f64, grav: &Gravity) -> f64 {
    unsafe {
        fteff_field_iron_(&Tb, &bfield, &grav.gs14)
    }
}

#[allow(non_snake_case)]
pub fn splint1(xa: &Vec<f64>, ya: &Vec<f64>, y2a: &Vec<f64>, i_n: i32, x: &Vec<f64>) -> Vec<f64> {
    let mut y = vec![0f64; x.len()];
    unsafe {
        splint1_(xa.as_ptr(), ya.as_ptr(), y2a.as_ptr(), &i_n, x.as_ptr(), y.as_mut_ptr());
    }
    y
}

#[allow(non_snake_case)]
pub fn spline1(x: &Vec<f64>, y: &Vec<f64>, i_n: i32, yp1: f64, ypn: f64) -> Vec<f64> {
    let mut y2 = vec![0f64; i_n as usize];
    unsafe {
        spline1_(x.as_ptr(), y.as_ptr(), &i_n, &yp1, &ypn, y2.as_mut_ptr());
    }
    y2
}

//#[allow(non_snake_case)]
//pub fn fteff_table(Tb: f64, Ts1: f64, Ts2: f64, grav: &Gravity, bound: &mut Bound, read: &mut bool) -> f64 {
//    let (Tb_acc0, f_TbTs) = (bound.Tb_acc0, bound.f_TbTs.as_str());
//    if !*read {
//        println!("-------------------------------------------------");
//        println!("Using envelope boundary condition from table");
//        println!("     {}", f_TbTs);
//        println!("WARNING:   No gs14 correction applied ! ");
//        println!("-------------------------------------------------");
//
//        // Read Netcdf file
//        let df = DataFrame::read_nc(f_TbTs).expect(&format!("Can't open {}", f_TbTs));
//        let jmax = df["jmax"][0];
//    }
//
//    unimplemented!()
//}