//! Yakovlev et al. functions
//!
//! Values from:
//! Yakovlev, Kaminker, & Gnedin, A&A 379, L5   (2001)
//! Kaminker, Yakovlev, & Gnedin, A&A 383, 1076 (2002)
//! Yakovlev, Kaminker, Haensel, & Gnedin, A&A 389, L24 (2002)

/// Tc_Ioffe
#[allow(non_snake_case)]
pub fn Tc_Ioffe(kf: f64, T0: f64, k0: f64, k1: f64, k2: f64, k3: f64) -> f64 {
    if kf > k0 && kf < k2 {
        let Tc = T0 * (kf - k0).powi(2) / ((kf - k0).powi(2) + k1.powi(2)) * (kf - k2).powi(2) / ((kf - k2).powi(2) + k3.powi(2));
        return 1e+9 * Tc;
    } else {
        1f64
    }
}
// =======================================================================
// Proton 1S0
// =======================================================================
#[allow(non_snake_case)]
pub fn Tc_Ioffe_1p(kf: f64) -> f64 {
    Tc_Ioffe(kf, 20.29, 0.0, 1.117, 1.241, 0.1473)
}

#[allow(non_snake_case)]
pub fn Tc_Ioffe_2p(kf: f64) -> f64 {
    Tc_Ioffe(kf, 17f64, 0f64, 1.117, 1.329, 0.1179)
}

#[allow(non_snake_case)]
pub fn Tc_Ioffe_3p(kf: f64) -> f64 {
    Tc_Ioffe(kf, 14.50, 0f64, 1.117, 1.518, 0.1179)
}

// =======================================================================
// Neutron 1S0
// =======================================================================
#[allow(non_snake_case)]
pub fn Tc_Ioffe_1ns(kf: f64) -> f64 {
    Tc_Ioffe(kf, 10.20, 0f64, 0.6, 1.45, 0.1)
}

#[allow(non_snake_case)]
pub fn Tc_Ioffe_2ns(kf: f64) -> f64 {
    Tc_Ioffe(kf, 7.9, 0f64, 0.3, 1.45, 0.01)
}

#[allow(non_snake_case)]
pub fn Tc_Ioffe_3ns(kf: f64) -> f64 {
    Tc_Ioffe(kf, 1800f64, 0f64, 21f64, 1.45, 0.4125)
}

// =======================================================================
// Neutron 3P2
// =======================================================================
#[allow(non_snake_case)]
pub fn Tc_Ioffe_1nt(kf: f64) -> f64 {
    Tc_Ioffe(kf, 6.461, 1f64, 1.961, 2.755, 1.3)
}

#[allow(non_snake_case)]
pub fn Tc_Ioffe_2nt(kf: f64) -> f64 {
    Tc_Ioffe(kf, 2f64, 1f64, 1.961, 2.755, 1.3)
}

#[allow(non_snake_case)]
pub fn Tc_Ioffe_3nt(kf: f64) -> f64 {
    Tc_Ioffe(kf, 15f64, 1f64, 1.961, 2.755, 1.3)
}