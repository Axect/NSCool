//! TODO
//!
//! ## To implement by hand
//! * `con_crust_e_phonon_Itoh`
//!     * `Code/Data_Files/con_crust_cryst_Itoh.dat`
//! * `con_crust` (Because of call `con_crust_e_phonon_Itoh`)

use std::os::raw::{c_double, c_int};
use crate::backend::inc::RhoLimits;
use std::f64::consts::PI;
use peroxide::{DataFrame, WithNetCDF};

#[allow(non_snake_case)]
#[link(name = "conductivity_crust")]
extern {
    pub fn con_crust_ee_(T: *const c_double, ne: *const c_double, lambda: *mut c_double, debug: *const c_double);
    pub fn con_crust_e_phonon_by_(T: *const c_double, rho: *const c_double, A: *const c_double, A1: *const c_double, Z: *const c_double, sigma: *mut c_double, debug: *const c_double, nu_e_s: *mut c_double, nu_e_l: *mut c_double, rhodrip: *const c_double);
    pub fn con_env_e_phon_ion_pbhy_(T: *const c_double, rho: *const c_double, A: *const c_double, A1: *const c_double, Z: *const c_double, sigma: *mut c_double, debug: *const c_double, nu_e_s: *mut c_double, nu_e_l: *mut c_double);
    pub fn con_env_e_phon_ion_gyp_(Temp: *const c_double, rho: *const c_double, A: *const c_double, A1: *const c_double, Z: *const c_double, sigma: *mut c_double, debug: *const c_double, nu_e_s: *mut c_double, nu_e_l: *mut c_double, rhodrip: *const c_double);
    pub fn get_lam_(s: *const c_double, w: *const c_double, Lam1: *mut c_double, Lam2: *mut c_double);
    pub fn exp_int_(x: *const c_double) -> c_double;
    pub fn oyaform_(BARD: *const c_double, Index: *const c_int, Z: *mut c_double, Anuc: *mut c_double, A: *mut c_double, xnuc: *mut c_double, xnuct: *mut c_double);
    pub fn con_crust_e_ion_itoh_(T: *const c_double, rho: *const c_double, A: *const c_double, A1: *const c_double, Z: *const c_double, sigma: *mut c_double, debug: *const c_double, nu_e_s: *mut c_double, nu_e_l: *mut c_double, rhodrip: *const c_double);
    pub fn con_crust_e_phonon_yu_(T: *const c_double, rho: *const c_double, A: *const c_double, A1: *const c_double, Z: *const c_double, sigma: *mut c_double, debug: *const c_double, nu_e_s: *mut c_double, nu_e_l: *mut c_double);
    pub fn con_crust_e_imp_yu_(T: *const c_double, rho: *const c_double, A: *const c_double, A1: *const c_double, Z: *const c_double, Q: *const c_double, sigma: *mut c_double, debug: *const c_double, nu_e_s: *mut c_double, nu_e_l: *mut c_double, rhodrip: *const c_double);
    pub fn coulomb_imp_yu_(x: *const c_double) -> c_double;
}

pub fn con_crust_ee(T: f64, ne: f64, debug: f64) -> f64 {
    let mut lambda = 0f64;
    unsafe {
        con_crust_ee_(&T, &ne, &mut lambda, &debug);
    }
    lambda
}

#[derive(Debug, Copy, Clone, Default)]
pub struct ConductOutput {
    pub sigma: f64,
    pub lambda: f64,
    pub nu_e_s: f64,
    pub nu_e_l: f64,
}

impl ConductOutput {
    fn new() -> Self {
        ConductOutput::default()
    }
}

#[allow(non_snake_case)]
pub fn con_crust_e_phonon_BY(T: f64, rho: f64, A: f64, A1: f64, Z: f64, debug: f64, rholim: RhoLimits) -> ConductOutput {
    let mut output = ConductOutput::new();
    unsafe {
        con_crust_e_phonon_by_(&T, &rho, &A, &A1, &Z, &mut output.sigma, &debug, &mut output.nu_e_s, &mut output.nu_e_l, &rholim.rho_drip);
    }
    output
}

#[allow(non_snake_case)]
pub fn con_env_e_phon_ion_PBHY(T: f64, rho: f64, A: f64, A1: f64, Z: f64, debug: f64) -> ConductOutput {
    let mut output = ConductOutput::new();
    unsafe {
        con_env_e_phon_ion_pbhy_(&T, &rho, &A, &A1, &Z, &mut output.sigma, &debug, &mut output.nu_e_s, &mut output.nu_e_l);
    }
    output
}

#[allow(non_snake_case)]
pub fn con_env_e_phon_ion_GYP(T: f64, rho: f64, A: f64, A1: f64, Z: f64, debug: f64, rholim: RhoLimits) -> ConductOutput {
    let mut output = ConductOutput::new();
    unsafe {
        con_env_e_phon_ion_gyp_(&T, &rho, &A, &A1, &Z, &mut output.sigma, &debug, &mut output.nu_e_s, &mut output.nu_e_l, &rholim.rho_drip);
    }
    output
}

#[allow(non_snake_case)]
pub fn get_lam(s: f64, w: f64) -> (f64, f64) {
    let (mut lam1, mut lam2) = (0f64, 0f64);
    unsafe {
        get_lam_(&s, &w, &mut lam1, &mut lam2);
    }
    (lam1, lam2)
}

#[allow(non_snake_case)]
pub fn exp_int(x: f64) -> f64 {
    unsafe {
        exp_int_(&x)
    }
}

#[allow(non_snake_case)]
pub fn con_crust_e_ion_Itoh(T: f64, rho: f64, A: f64, A1: f64, Z: f64, debug: f64, rholim: RhoLimits) -> ConductOutput {
    let mut output = ConductOutput::new();
    unsafe {
        con_crust_e_ion_itoh_(&T, &rho, &A, &A1, &Z, &mut output.sigma, &debug, &mut output.nu_e_s, &mut output.nu_e_l, &rholim.rho_drip);
    }
    output
}

#[allow(non_snake_case)]
pub fn con_crust_e_phonon_YU(T: f64, rho: f64, A: f64, A1: f64, Z: f64, debug: f64) -> ConductOutput {
    let mut output = ConductOutput::new();
    unsafe {
        con_crust_e_phonon_yu_(&T, &rho, &A, &A1, &Z, &mut output.sigma, &debug, &mut output.nu_e_s, &mut output.nu_e_l);
    }
    output
}

#[allow(non_snake_case)]
pub fn con_crust_e_imp_YU(T: f64, rho: f64, A: f64, A1: f64, Z: f64, Q: f64, debug: f64, rholim: RhoLimits) -> ConductOutput {
    let mut output = ConductOutput::new();
    unsafe {
        con_crust_e_imp_yu_(&T, &rho, &A, &A1, &Z, &Q, &mut output.sigma, &debug, &mut output.nu_e_s, &mut output.nu_e_l, &rholim.rho_drip);
    }
    output
}

#[allow(non_snake_case)]
pub fn coulomb_imp_YU(x: f64) -> f64 {
    unsafe {
        coulomb_imp_yu_(&x)
    }
}

const ALPHA: [f64; 4] = [0.6898, 14.4310, -182.2730, 449.1520];
const BETA: [f64; 4] = [0.7665, 10.0813, -115.9180, 201.7040];

#[allow(non_snake_case)]
pub fn con_crust_e_phonon_Itoh(T: f64, rho: f64, A: f64, A1: f64, Z: f64, debug: f64, df: &DataFrame) -> ConductOutput {
    let j_size: usize = 1050;

    if debug == 1.2 {
        println!("Entering con_crust_e_phonon: T, rho = {}, {}", T, rho);
    }

    let Is171 = &df["Is171"];
    let Ik171 = &df["Ik171"];
    let Is5000 = &df["Is5000"];
    let Ik5000 = &df["Ik5000"];

    let rho6 = rho / 1e+6;
    let T8 = T / 1e+8;
    let gamma = 0.2275 * Z.powi(2) * (rho6 / A).powf(1f64 / 3f64) / T8;
    let jfit = (100f64 * (rho / 1e+4).log10()).max(0f64) as usize;
    let gam = 7.832e-2 * Z * (rho6 / A / A1).sqrt() / T8;
    let x2 = 1.018 * (rho6 * Z / A).powf(2f64 / 3f64);

    let mut v = ALPHA[0];
    let mut w = BETA[0];
    let gamthird = gamma.powf(1f64 / 3f64);
    for i in 1 .. 4 {
        v[i] += ALPHA[i] / gamthird.powi(i as i32);
        w[i] += BETA[i] / gamthird.powi(i as i32);
    }

    //**** From Erratum in ApJ 404 (1993) p. 418
    //       I_s=v*Is171(jfit)+(1-v)*Is5000(jfit)
    //       I_k=w*Ik171(jfit)+(1-w)*Ik5000(jfit)
    let I_s = (1f64 - v) * Is171[jfit] + v * Is5000[jfit];
    let I_k = (1f64 - w) * Ik171[jfit] + w * Ik5000[jfit];

    let G_0 = 13f64 / (1f64 + 0.0174 * gam.powi(2)).sqrt();
    let G_2 = gam.powi(2) / PI.powi(2) / (1f64 + 0.0118 * gam.powi(2)).powf(1.5);

    let F_s = I_s * G_0;
    let F_k = I_s * G_0 + I_k * G_2;
    let nu_s = 9.554e+16 * T8 * (1f64 + 1f64 / x2).sqrt() * F_s;
    let nu_k = 9.554e+16 * T8 * (1f64 + 1f64 / x2).sqrt() * F_k;

    let sigma = 1.525e+20 * Z / A * rho6 / (1f64 + x2).sqrt() * (1e+18 / nu_s);
    let lambda = 4.146e+15 * Z / A * rho6 / (1f64 + x2).sqrt() * T8 * (1e+18 / nu_k);

    if debug == 1.2 {
        println!("Exiting con_crust_e_phonon: sigma, lambda = {}, {}", sigma, lambda);
    }

    let mut output = ConductOutput::new();
    output.sigma = sigma;
    output.lambda = lambda;
    output.nu_e_s = nu_s;
    output.nu_e_l = nu_k;

    output
}
