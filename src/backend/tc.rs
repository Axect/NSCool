//! Neutron 1S0 superfluidity

use std::os::raw::{c_int, c_double};

#[link(name = "tc")]
extern {
    pub fn tcn1_sfb_(k: *const c_double) -> c_double;
    pub fn tcn1_ccdk_(k: *const c_double) -> c_double;
    pub fn tcn1_wap_(k: *const c_double) -> c_double;
    pub fn tcn1_GC_(k: *const c_double) -> c_double;
    pub fn tcn1_gipsf_(k: *const c_double) -> c_double;
    pub fn tcn1_t72_(k: *const c_double) -> c_double;
    pub fn tcn1_ns_(k: *const c_double) -> c_double;
    pub fn tcn1_t84_(k: *const c_double) -> c_double;
    pub fn tcn1_ao_(k: *const c_double) -> c_double;
    pub fn tcn1_ccks_var_(k: *const c_double) -> c_double;
    pub fn tcn1_ccks_cbf_(k: *const c_double) -> c_double;
    pub fn tcn1_awp_2_(k: *const c_double) -> c_double;
    pub fn tcn1_awp_3_(k: *const c_double) -> c_double;
    pub fn tcn1_bbllp_(k: *const c_double) -> c_double;
    pub fn tcn1_sclbl96_(k: *const c_double) -> c_double;
    pub fn tcn1_sclbl96_pol_(k: *const c_double) -> c_double;
    pub fn tcn3_hgrr_(k: *const c_double) -> c_double;
    pub fn tcn3_ao_(k: *const c_double) -> c_double;
    pub fn tcn3_t72_(k: *const c_double) -> c_double;
    pub fn tcn3_t72_m1_(k: *const c_double) -> c_double;
    pub fn tcn3_ao_m1_(k: *const c_double) -> c_double;
    pub fn tcn3_bcll92_(k: *const c_double) -> c_double;
    pub fn tcn3_eehjo96_nr_(k: *const c_double) -> c_double;
    pub fn tcn3_eehjo96_r_(k: *const c_double) -> c_double;
    pub fn tcp1_ccy_ms_(k: *const c_double) -> c_double;
    pub fn tcp1_ccy_ps_(k: *const c_double) -> c_double;
    pub fn tcp1_ccdk_(k: *const c_double) -> c_double;
    pub fn tcp1_t73_(k: *const c_double) -> c_double;
    pub fn tcp1_ns_(k: *const c_double) -> c_double;
    pub fn tcp1_ao_(k: *const c_double) -> c_double;
    pub fn tcp1_bcll92_(k: *const c_double) -> c_double;
    pub fn tcp1_eeho_(k: *const c_double) -> c_double;
    pub fn tcla1_bb_(k: *const c_double, nbar: *const c_double) -> c_double;
    pub fn spline_here_(x: *const c_double, y: *const c_double, i_n: *const c_int, yp1: *const c_double, ypn: *const c_double, y2: *mut c_double);
}

pub fn tcn1_sfb(k: f64) -> f64 {
    unsafe {
        tcn1_sfb_(&k)
    }
}

pub fn tcn1_ccdk(k: f64) -> f64 {
    unsafe {
        tcn1_ccdk_(&k)
    }
}

pub fn tcn1_wap(k: f64) -> f64 {
    unsafe {
        tcn1_wap_(&k)
    }
}

#[allow(non_snake_case)]
pub fn tcn1_GC(k: f64) -> f64 {
    unsafe {
        tcn1_GC_(&k)
    }
}

pub fn tcn1_gipsf(k: f64) -> f64 {
    unsafe {
        tcn1_gipsf_(&k)
    }
}

pub fn tcn1_t72(k: f64) -> f64 {
    unsafe {
        tcn1_t72_(&k)
    }
}

pub fn tcn1_ns(k: f64) -> f64 {
    unsafe {
        tcn1_ns_(&k)
    }
}

pub fn tcn1_t84(k: f64) -> f64 {
    unsafe {
        tcn1_t84_(&k)
    }
}

pub fn tcn1_ao(k: f64) -> f64 {
    unsafe {
        tcn1_ao_(&k)
    }
}
//pub fn tcn1_ccks_var_(k: f64) -> f64 { }
//pub fn tcn1_ccks_cbf_(k: f64) -> f64 { }
//pub fn tcn1_awp_2_(k: f64) -> f64 { }
//pub fn tcn1_awp_3_(k: f64) -> f64 { }
//pub fn tcn1_bbllp_(k: f64) -> f64 { }
//pub fn tcn1_sclbl96_(k: f64) -> f64 { }
//pub fn tcn1_sclbl96_pol_(k: f64) -> f64 { }
//pub fn tcn3_hgrr_(k: f64) -> f64 { }
//pub fn tcn3_ao_(k: f64) -> f64 { }
//pub fn tcn3_t72_(k: f64) -> f64 { }
//pub fn tcn3_t72_m1_(k: f64) -> f64 { }
//pub fn tcn3_ao_m1_(k: f64) -> f64 { }
//pub fn tcn3_bcll92_(k: f64) -> f64 { }
//pub fn tcn3_eehjo96_nr_(k: f64) -> f64 { }
//pub fn tcn3_eehjo96_r_(k: f64) -> f64 { }
//pub fn tcp1_ccy_ms_(k: f64) -> f64 { }
//pub fn tcp1_ccy_ps_(k: f64) -> f64 { }
//pub fn tcp1_ccdk_(k: f64) -> f64 { }
//pub fn tcp1_t73_(k: f64) -> f64 { }
//pub fn tcp1_ns_(k: f64) -> f64 { }
//pub fn tcp1_ao_(k: f64) -> f64 { }
//pub fn tcp1_bcll92_(k: f64) -> f64 { }
//pub fn tcp1_eeho_(k: f64) -> f64 { }
//pub fn tcla1_bb_(k: f64, nbar: f64) -> f64 {
//
//}



//#[derive(Debug, Copy, Clone)]
//pub enum TCN1 {
//    SFB,
//    CCDK,
//}
//
//pub const SFB_K0: [f64; 18] = [
//    0.000, 0.100, 0.200, 0.300, 0.400, 0.500,
//    0.600, 0.700, 0.800, 0.900, 1.000, 1.100,
//    1.175, 1.250, 1.300, 1.350, 1.400, 1.450
//];
//pub const SFB_D0: [f64; 18] = [
//    0.000, 0.000, 0.090, 0.210, 0.350, 0.490,
//    0.610, 0.720, 0.790, 0.780, 0.700, 0.570,
//    0.440, 0.280, 0.190, 0.100, 0.030, 0.000
//];



//pub fn spline_here(x: &Vec<f64>, y: &Vec<f64>, i_n: usize, yp1: f64) -> Vec<f64> {
//    let jmax = 100usize;
//    let mut y2 = vec![0f64; i_n];
//    let mut u = vec![0f64; i_n];
//    if yp1 <= 1e+30 {
//        y2[0] = -0.5;
//        u[0] = (3f64 / (x[1] - x[0])) * ((y[1] - y[0]) / (x[1] - x[0]) - yp1);
//    }
//    for i in 1 .. i_n-1 {
//        let sig = (x[i] - x[i-1]) / (x[i+1] - x[i-1]);
//        let p = sig * y2[i-1] + 2f64;
//        y2[i] = (sig - 1f64) / p;
//        u[i] = (6. * ((y[i + 1] - y[i]) / (x[i + 1] - x[i]) - (y[i] - y[i - 1])
//            / (x[i] - x[i - 1])) / (x[i + 1] - x[i - 1]) - sig * u[i - 1]) / p;
//    }
//
//    let (qn, un) = if ypn > 1e+30 {
//        (0f64, 0f64)
//    } else {
//        (0.5, (3. / (x[i_n-1] - x[i_n-2])) * (ypn - (y[i_n-1] - y[i_n-2]) / (x[i_n-1] - x[i_n-2])))
//    };
//
//    for ik in (0 .. i_n-1).rev() {
//        y2[ik] = y2[ik] * y2[ik+1] + u[ik];
//    }
//
//    y2
//}