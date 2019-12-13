use crate::backend::inc::{Accretion, ProfileStar, Gravity, AccretionType};

#[allow(non_snake_case)]
pub fn initialize_accretion_rate(acc: &mut Accretion, prof: &mut ProfileStar, grav: &Gravity) {
    let mut m_dot1 = 0f64;
    let mut eta = 0f64;
    let mut X = 0f64;
    let mut opac = 0f64;
    let (mut F_Edd, mut L_Edd, mut M_Edd) = (0f64, 0f64, 0f64);

    if acc.m_dot0 > 1e+10 {         // m_dot in g/sec
        m_dot1 = acc.m_dot0;
    } else if acc.m_dot0 < 1e-6 {   // m_dot in Msun/yr
        m_dot1 = acc.m_dot0 * 2e+33 / 3.15576e+7;
    } else {                    // m_dot in unit of M_Edd
        eta = acc.eta_Edd;      // Efficiency of mass -> energy
        X = acc.X_Edd;          // Hydrogen fraction
        opac = 6.023e+23 * 0.665e-24 * (1f64 + X) / 2f64;
        F_Edd = 3e+10 * (1e+14 * grav.gs14) / opac;
        M_Edd = L_Edd / (3e+10).powi(2) / eta;
        m_dot1 = m_dot0 * M_Edd;
    }

    let mut mass_acc = 0f64;
    let mut tau_acc_r = 0f64;

    match acc.i_acc {
        AccretionType::None => (),
        AccretionType::TransientFRED => {
            // t_acc0 = beginning of accretion
            // t_acc1 = duration of each accretion cycle
            // t_acc2 = tau_acc_r = duration of exponential rise
            tau_acc_r = acc.t_acc2;
            if *m_dot0 < 1f64 {
                // In this case m_dot0 is actually DelM in Mo
                let coeff = 1f64 / (1f64.exp() - 1f64) + (1f64 - (acc.t_acc2 / acc.t_acc1).powf(acc.alpha_acc - 1f64)) / (acc.alpha_acc - 1f64);
                acc.m_dot0 /= t_acc2 / coeff * 2e+33;
            }
            acc.m_dot_max = acc.m_dot0;
            acc.m_dot_ris = acc.m_dot_max / (1f64 - (-1f64).exp());
            let macc_r = acc.t_acc2 * acc.m_dot_ris / 1f64.exp();
            let macc_d = acc.t_acc2 * acc.m_dot_max / (acc.alpha_acc - 1f64) * (1f64 - (acc.t_acc2 / acc.t_acc1).powf(acc.alpha_acc - 1f64));
            acc.m_dot_av = (macc_r + macc_d) / acc.t_acc1;
            acc.m_dot_ini = acc.m_dot_av;
            acc.m_dot_quiet = 0f64;
            let coeff_ = 3.15e+7 / 2e+33;
        }
        AccretionType::TransientSTEP => {
            acc.m_dot_max = m_dot1;
            acc.m_dot_av = m_dot1 * acc.t_acc2 / acc.t_acc1;
            acc.m_dot_ini = acc.m_dot_av;
            acc.m_dot_quiet = 0f64;
            let coeff_ = 3.15e+7 / 2e+33;
        }
    }
}