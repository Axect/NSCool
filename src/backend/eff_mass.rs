/// mstn_pref
///
/// # Description
/// > calculate the neutron effective mass, prefered values from
/// >  analysis of all data
/// > k is the fermi momentum in fm^-1 and m*=mst is given as the m*/m
pub fn mstn_pref(k: f64) -> f64 {
    1f64.min(1.09 - 0.11 * k)
}

const MSTN_AWP_3_K0: [f64; 6] = [0.6, 0.8, 1.0, 1.2, 1.4, 1.6];
const MSTN_AWP_3_M0: [f64; 6] = [1.004, 0.997, 0.982, 0.961, 0.936, 0.910];
const MSTN_AWP_3_D_K0: [f64; 6] = [0.6, 0.8, 1.0, 1.2, 1.4, 1.6];
const MSTN_AWP_3_D_M0: [f64; 6] = [0.958, 0.931, 0.915, 0.894, 0.871, 0.851];
const MSTN_CCKS_BJ_VAR_K0: [f64; 11] = [0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3];
const MSTN_CCKS_BJ_VAR_M0: [f64; 11] = [0.99, 0.98, 0.96, 0.94, 0.92, 0.90, 0.88, 0.85, 0.82, 0.78, 0.75];
const MSTN_CCKS_R_CBF_K0: [f64; 18] = [0.30, 0.40, 0.50, 0.60, 0.70, 0.80, 0.90, 1.00, 1.10, 1.20, 1.30, 1.50, 1.75, 2.00, 2.25, 2.50, 2.75, 3.00];
const MSTN_CCKS_R_CBF_M0: [f64; 18] = [1.10, 1.10, 1.095, 1.09, 1.08, 1.07,1.06, 1.05, 1.04, 1.02, 1.00, 0.96, 0.89, 0.82, 0.75, 0.69, 0.65, 0.64];
const MSTN_CCKS_R_CBF_M2: [f64; 18] = [4.97e-01, -9.95e-01, 4.81e-01, -9.31e-01, 2.41e-01, -3.40e-02, -1.05e-01, 4.54e-01, -1.71e+00, 3.93e-01, 1.39e-01, -6.13e-01, 1.76e-01, -9.23e-02, 1.93e-01, 2.80e-01, 6.06e-01, 1.77e-01];
const MSTN_CCKS_BJ_CBF_K0: [f64; 18] = [0.30, 0.40, 0.50, 0.60, 0.70, 0.80, 0.90, 1.00, 1.10, 1.20, 1.30, 1.50, 1.75, 2.00, 2.25, 2.50, 2.75, 3.00];
const MSTN_CCKS_BJ_CBF_M0: [f64; 18] = [0.83, 0.91, 0.97, 1.01, 1.03, 1.04, 1.04, 1.03, 1.01, 0.98, 0.94, 0.87, 0.77, 0.67, 0.59, 0.53, 0.47, 0.43];
const MSTN_CCKS_BJ_CBF_M2: [f64; 18] = [2.92e+01, -1.04e+01, 3.37e-01, -2.97e+00, -4.73e-01, -1.14e+00, -9.52e-01, -1.05e+00, -8.58e-01, -1.52e+00, 9.38e-01, -5.55e-01, 4.79e-02, 3.64e-01, 4.17e-01, -1.12e-01, 3.21e-02, 1.90e+00];
const MSTN_T_K0: [f64; 18] = [0.150, 0.197, 0.242, 0.329, 0.446, 0.518, 0.613, 0.811, 0.972, 1.090, 1.190, 1.300, 1.370, 1.480, 1.620, 1.920, 2.130, 2.400];
const MSTN_T_M0: [f64; 18] = [1.00, 1.00, 1.00, 1.00, 1.00, 0.99, 0.98, 0.96, 0.95, 0.94, 0.93, 0.91, 0.89, 0.86, 0.82, 0.78, 0.75, 0.70];
const MSTN_BKS_K0: [f64; 10] = [0.62, 0.88, 1.05, 1.20, 1.37, 1.48, 1.62, 1.92, 2.13, 2.40];
const MSTN_BKS_M0: [f64; 10] = [1.02, 0.99, 0.96, 0.94, 0.91, 0.89, 0.87, 0.83, 0.81, 0.80];
const MSTN_BBLLP_K0: [f64; 5] = [0.250, 0.510, 0.810, 1.080, 1.300];
const MSTN_BBLLP_M0: [f64; 5] = [0.995, 0.980, 0.955, 0.925, 0.910];

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case)]
pub enum MSTN {
    MSTN_AWP_3,
    MSTN_AWP_3_D,
    MSTN_CCKS_BJ_VAR,
    MSTN_CCKS_R_CBF,
    MSTN_CCKS_BJ_CBF,
    MSTN_T,
    MSTN_BKS,
    MSTN_BBLLP,
}

pub fn mstn(species: MSTN, k: f64) -> f64 {
    let (k0, m0, m2, i_max) = match species {
        MSTN::MSTN_AWP_3 => (Vec::from(&MSTN_AWP_3_K0), Vec::from(&MSTN_AWP_3_M0), Vec::<f64>::new(), 6),
        MSTN::MSTN_AWP_3_D => (Vec::from(&MSTN_AWP_3_D_K0), Vec::from(&MSTN_AWP_3_D_M0), Vec::<f64>::new(), 6),
        MSTN::MSTN_CCKS_BJ_VAR => (Vec::from(&MSTN_CCKS_BJ_VAR_K0), Vec::from(&MSTN_CCKS_BJ_VAR_M0), Vec::<f64>::new(), 11),
        MSTN::MSTN_CCKS_R_CBF => (Vec::from(&MSTN_CCKS_R_CBF_K0), Vec::from(&MSTN_CCKS_R_CBF_M0), Vec::from(&MSTN_CCKS_R_CBF_M2), 18),
        MSTN::MSTN_CCKS_BJ_CBF => (Vec::from(&MSTN_CCKS_BJ_CBF_K0), Vec::from(&MSTN_CCKS_BJ_CBF_M0), Vec::from(&MSTN_CCKS_BJ_CBF_M2), 18),
        MSTN::MSTN_T => (Vec::from(&MSTN_T_K0), Vec::from(&MSTN_T_M0), Vec::<f64>::new(), 18),
        MSTN::MSTN_BKS => (Vec::from(&MSTN_BKS_K0), Vec::from(&MSTN_BKS_M0), Vec::<f64>::new(), 10),
        MSTN::MSTN_BBLLP => (Vec::from(&MSTN_BBLLP_K0), Vec::from(&MSTN_BBLLP_M0), Vec::<f64>::new(), 5),
    };

    let mut i1 = 1usize;
    let mut i2 = i_max;
    let mut i: usize;

    if k <= k0[0] {
        return m0[0];
    } else if k >= k0[i_max-1] {
        return m0[i_max-1];
    } else {
        loop {
            if (i2 - i1) > 1 {
                i = (i1 + i2) / 2;
                if (k0[i - 1] > k) {
                    i2 = i;
                } else {
                    i1 = i;
                }
            } else {
                break;
            }
        }
        i1 -= 1;
        i2 -= 1;
        let del_k = k0[i2] - k0[i1];
        let a = (k0[i2] - k) / del_k;
        let b = (k - k0[i1]) / del_k;
        let m = match species {
            MSTN::MSTN_AWP_3 => a * m0[i1] + b * m0[i2],
            MSTN::MSTN_AWP_3_D => a * m0[i1] + b * m0[i2],
            MSTN::MSTN_CCKS_BJ_VAR => a * m0[i1] + b * m0[i2],
            MSTN::MSTN_CCKS_R_CBF => a * m0[i1] + b * m0[i2] + ((a.powi(3) - a) * m2[i1] + (b.powi(3) - b) * m2[i2]) * del_k.powi(2) / 6f64,
            MSTN::MSTN_CCKS_BJ_CBF => a * m0[i1] + b * m0[i2] + ((a.powi(3) - a) * m2[i1] + (b.powi(3) - b) * m2[i2]) * del_k.powi(2) / 6f64,
            MSTN::MSTN_T => a * m0[i1] + b * m0[i2],
            MSTN::MSTN_BKS => a * m0[i1] + b * m0[i2],
            MSTN::MSTN_BBLLP => a * m0[i1] + b * m0[i2],
        };

        return m;
    }
}