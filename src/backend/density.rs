//use std::f64::consts::PI;
use peroxide::*;
//use std::f64::EPSILON;

/// Density
///
/// # Description
/// > given T,P,A,Z, calculates Rho: uses Newton's method
/// > and Rho as input is taken as a first guess
#[allow(non_snake_case)]
pub fn density(T: f64, P: f64, A: f64, Z: f64, Rho: f64, Saved: &mut Vec<f64>) -> f64 {
    let eps = 1e-3;
    let mut rho = Rho;
    loop {
        let Rho0 = rho;
        let Pre0 = pressure(T, Rho0, A, Z, Saved);
        let Rho1 = (1f64 + eps) * Rho0;
        let Pre1 = pressure(T, Rho1, A, Z, Saved);
        let f = Pre0 - P;
        let f1 = (Pre1 - Pre0) / (Rho1 - Rho0);
        let dRho = - f / f1;
        rho = Rho0 + dRho;
        if (dRho / rho).abs() < 1e-5 {
            return rho;
        }
    }
}

#[allow(non_snake_case)]
pub fn pressure(T: f64, Rho: f64, A: f64, Z: f64, Saved: &mut Vec<f64>) -> f64 {
    use crate::NA;
//    use crate::C as c;
//    use crate::HB as hb;
    use crate::KB as kb;
//    use crate::ME as me;

    // =========================================================================
    // Calculate the electron density and ionization
    // =========================================================================
    let ne = Rho * NA * Z / A;
    let nion = Rho * NA / A;
    let Zeff = ne / nion;

    // =========================================================================
    // Calculate the ionic pressure
    // =========================================================================
    let gamma = 2.273E+5 * Zeff.powi(2) * (Rho / A).powf(1f64/3f64) / T;
    let Uion = if gamma >= 210f64 {
        1.5 - 0.895929 * gamma + 3225f64 / gamma.powi(2)
    } else {
        -0.897744 * gamma + 0.95043 * gamma.powf(0.25) + 0.18956 / gamma.powf(0.25) - 0.81487
    };
    let Pion = nion * kb * T * (1f64 + Uion / 3f64);
    let Pel = P_electron(T, ne, Saved);
    let Pres = Pel + Pion;

    return Pres;
}

/// Pressure of Electron
///
/// # Description:
/// > Calculates the pressure of a perfect electron gas (Fermi-Dirac)
/// > From Eggleton, Faulkner & Flannery, A&A23 (1973), p. 325-330
/// > for the approx. which gives thermodynamically consistent D,U & P
/// >
/// >    CHECKED on nov. 24 1990 and again on July 15 2005
/// >       that it gives the correct perfect gas and
/// >  degenerate relativistic and non-relativistic limits :
/// >
/// >    PDEGNR=(3.*PI**2)**(2./3.)/5.*(HB*NE)*(HB/ME)*NE**(2./3.)
/// >    PDEGR=(3.*PI**2)**(1./3.)/4.*(HB*NE)*(  C  )*NE**(1./3.)
/// >    PPERF=NE*KB*T
#[allow(non_snake_case)]
pub fn P_electron(T: f64, NE: f64, Saved: &mut Vec<f64>) -> f64 {
    use crate::NA;
    let epsilon: f64 = 1e-12;
    let mut DS = vec![0f64; 4];
    let CD = matrix(vec![
        2.315472, 7.1286600, 7.5049980, 2.665350,
        7.837752, 23.507934, 23.311317, 7.987465,
        9.215560, 26.834068, 25.082745, 8.020509,
        3.693280, 10.333176, 9.1689600, 2.668248,
    ], 4, 4, Col);
    let CP = matrix(vec![
        2.315472, 6.7481040, 6.5649120, 2.132280,
        7.837752, 21.439740, 19.080088, 5.478100,
        9.215560, 23.551504, 19.015888, 4.679944,
        3.693280, 8.8598680, 6.5007120, 1.334124
    ], 4, 4, Col);
//    let CU = matrix(vec![
//        3.4732080, 10.122156, 9.8473680, 3.1984200,
//        16.121172, 43.477194, 37.852852, 10.496830,
//        23.971040, 60.392810, 47.782844, 11.361074,
//        11.079840, 26.579604, 19.502136, 4.0023720
//    ], 4, 4, Col);
    let NEHAT = NE / 1.7595E+30;

    let T1 = T / 5.93E+9;
    let mut F1 = 1E-3 * (NE / NA) * (1E+7 / T).powi(3);

    if Saved.len() == 0 {
        Saved.push(0f64); // OLDNE
        Saved.push(0f64); // OLDT
        Saved.push(0f64); // OLDF1
    }

    if (Saved[0] - NE).abs() / NE <= 5e-1 && (Saved[1] - T).abs() / T <= 5e-1 {
        F1 = Saved[2].clone();
    }

    loop {
        let F = F1.abs();
        let G = T1 * (F + 1f64).sqrt();
        let PF = F + 1f64;
        let PG = G + 1f64;

        for i in 0 .. 4 {
            DS[i] = CD[(i, 0)] + (CD[(i, 1)] + CD[(i, 2)] * G + CD[(i, 3)] * G.powi(2)) * G;
            DS[i] /=PF.powi(3);
        }
        let SUM1 = DS[0] + (DS[1] + DS[2]*F + DS[3]*F.powi(2)) * F;

        for i in 1 .. 4 {
            DS[i] = CD[(i, 0)] + (CD[(i, 1)] + CD[(i, 2)] * G + CD[(i, 3)]* G.powi(2)) * G;
            DS[i] /= PF.powi(3);
        }
        let SUM2 = DS[1] + (2f64 * DS[2] + 3f64 * DS[3] * F) * F;

        for i in 0 .. 4 {
            DS[i] = (CD[(i, 1)] + 2f64 * CD[(i, 2)] * G + 3f64 * CD[(i, 3)] * G.powi(2)) * G;
            DS[i] /= PF.powi(3);
        }
        let SUM3 = DS[0] + (DS[1] + DS[2] * F + DS[3] * F.powi(2)) * F;

        let COEF: f64;
        let NEF: f64;
        let NEF1: f64;

        if F < 1f64 {
            COEF = (F / PF) * (G / PG).powf(1.5);
            NEF = COEF * SUM1;
            NEF1 = (COEF / F - 3.25 * COEF / PF - 0.75 * COEF * G / PF / PG) * SUM1 + COEF * (SUM2 + 0.5 / PF * SUM3);
        } else {
            COEF = (F / PF) * (G / PG).powf(1.5);
            NEF = COEF * SUM1;
            NEF1 = COEF * (
                (1f64 / F - 3.25 / PF - 0.75 * G / (PF * PG)) * SUM1 + SUM2 + 0.5 / PF * SUM3
            );
        }

        F1 = F + (NEHAT - NEF) / NEF1;

        if (NEF - NEHAT).abs() / NEHAT.abs() > epsilon || (F1 - F).abs() / F1.abs() > epsilon {
            continue;
        } else {
            break;
        }
    }

    // =========================================================================
    // Calculate pressure
    // =========================================================================
    let G1 = T1 * (1f64 + F1).sqrt();

    for i in 0 .. 4 {
        DS[i] = CP[(i, 0)] + (CP[(i, 1)] + CP[(i, 2)] * G1 + CP[(i, 3)] * G1.powi(2)) * G1;
    }
    let SUMP1 = DS[0] + (DS[1] + DS[2] * F1 + DS[3] * F1.powi(2)) * F1;

    let P1 = if F1 < 1f64 {
        1.44E+24 * F1 * G1.powf(2.5) / (1f64 + F1).powi(4) / (1f64 + G1).powf(1.5) * SUMP1
    } else {
        1.44E+24 * (F1.powf(0.25) / (1f64 + F1)).powi(4) * G1 * (G1 / (1f64 + G1)).powf(1.5) * SUMP1
    };

    Saved[0] = NE;
    Saved[1] = T;
    Saved[2] = F1;

    return P1;
}